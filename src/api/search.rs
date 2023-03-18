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

use entity::music;
use sea_orm::{prelude::*, Set};

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
    pub title: String,
    pub artist: String,
    pub id: u64,
}

impl From<Track> for SearchMusic {
    fn from(music: Track) -> Self {
        Self {
            id: music.id,
            title: music.title,
            artist: music.artist.name,
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

    // TODO: check if room exists

    let response = state
        .search_client
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
    music_id: u64,
) -> Result<music::Model, DbErr> {
    let music = music::Entity::find()
        .filter(music::Column::Id.eq(music_id as i64)) // TODO: fix this
        .one(&state.db)
        .await?;

    match music {
        Some(music) => Ok(music),
        None => {
            let tract = state
                .tracks_client
                .get(&music_id.to_string())
                .await
                .map_err(|e| {
                    log::error!("Failed to get music: {}", e);
                    DbErr::Custom(e.to_string())
                })?;
            let music = music::ActiveModel {
                id: Set(music_id as i64), // TODO: fix this
                title: Set(tract.title),
                artist: Set(tract.artist.name),
            };
            music.insert(&state.db).await
        }
    }
}
