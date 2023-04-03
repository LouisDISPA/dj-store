use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use deezer_rs::track::Track;
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use entity::{music, room};
use sea_orm::{prelude::*, IntoActiveModel};

use crate::utils::{
    jwt::{Role, User},
    room_id::RoomID,
};

use super::state::ApiState;
#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMusic {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub preview_url: Option<String>,
    pub image_hash: Option<String>,
}

impl From<Track> for SearchMusic {
    fn from(music: Track) -> Self {
        Self {
            id: music.id,
            title: music.title,
            artist: music.artist.name,
            preview_url: Some(music.preview),
            image_hash: music.md5_image,
        }
    }
}

impl From<SearchMusic> for music::Model {
    fn from(music: SearchMusic) -> Self {
        Self {
            id: music.id,
            title: music.title,
            artist: music.artist,
            preview_url: music.preview_url,
            image_hash: music.image_hash,
        }
    }
}

#[derive(Error, Display, Debug)]
pub enum SearchError {
    /// Room not found
    RoomNotFound,
    /// User not in room.
    UserNotInRoom,
    /// Internal error
    InternalError,
}

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        use SearchError::*;
        let status = match self {
            UserNotInRoom => StatusCode::UNAUTHORIZED,
            RoomNotFound => StatusCode::UNAUTHORIZED,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

// TODO: prevent user from searching too much
pub async fn search(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    Query(request): Query<SearchRequest>,
    user: User,
) -> Result<Json<Vec<SearchMusic>>, SearchError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(SearchError::UserNotInRoom);
    }

    room::Entity::find()
        .filter(room::Column::PublicId.eq(room_id.value()))
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to get room: {}", e);
            SearchError::InternalError
        })?
        .ok_or(SearchError::RoomNotFound)?;

    let response = state
        .deezer_client
        .search()
        .get_tracks(&request.query)
        .await
        .map_err(|e| {
            log::error!("Failed to search music: {}", e);
            SearchError::InternalError
        })?;

    let musics = response.data.into_iter().map(SearchMusic::from).collect();

    Ok(Json(musics))
}

pub async fn get_music_or_store_music(
    state: &ApiState,
    music_id: i64,
) -> Result<music::Model, DbErr> {
    let music = music::Entity::find()
        .filter(music::Column::Id.eq(music_id))
        .one(&state.db)
        .await?;

    match music {
        Some(music) => Ok(music),
        None => {
            let music = state
                .deezer_client
                .track()
                .get(&music_id.to_string())
                .await
                .map(SearchMusic::from)
                .map(music::Model::from)
                .map_err(|e| {
                    log::error!("Failed to get music: {}", e);
                    DbErr::Custom(e.to_string())
                })?;
            let music = music.into_active_model();
            music.insert(&state.db).await
        }
    }
}
