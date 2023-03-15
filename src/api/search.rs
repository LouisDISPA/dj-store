use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use musicbrainz_rs::entity::recording::RecordingSearchQuery;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::{
    jwt::{Role, User},
    room_id::RoomID,
};

use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMusic {
    pub title: String,
    pub artist: Option<String>,
    pub id: String,
}

impl From<Recording> for SearchMusic {
    fn from(music: Recording) -> Self {
        Self {
            id: music.id,
            title: music.title,
            artist: music
                .artist_credit
                .as_deref()
                .and_then(<[_]>::first)
                .map(|a| a.name.clone()),
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
    Path(room_id): Path<RoomID>,
    Query(request): Query<SearchRequest>,
    user: User,
) -> Result<Json<Vec<SearchMusic>>, SearchError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(SearchError::UserNotInRoom);
    }

    // TODO: check if room exists

    let query = RecordingSearchQuery::query_builder()
        .recording(&request.query)
        .build();

    let result = Recording::search(query).execute().await.map_err(|e| {
        log::error!("Failed to search music: {}", e);
        SearchError::InternalError
    })?;

    // let mut musics = Vec::with_capacity(result.entities.len());

    // for recording in result.entities {

    // }

    let musics = result.entities.into_iter().map(SearchMusic::from).collect();

    Ok(Json(musics))
}
