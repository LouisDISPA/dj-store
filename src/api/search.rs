use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    model::{Music, Role, User, ROOMS},
    utils::lastfm::LastFmResult,
};

use super::{room_id::RoomID, AppState};

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMusic {
    pub title: String,
    pub artist: String,
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub musics: Vec<SearchMusic>,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Error, Display, Debug)]
pub enum SearchError {
    /// Room not found
    RoomNotFound,
    /// User not in room.
    UserNotInRoom,
    /// Last.fm error: {0}
    LastFmError(String),
    /// Internal error
    InternalError,
}

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        use SearchError::*;
        let status = match self {
            UserNotInRoom => StatusCode::UNAUTHORIZED,
            RoomNotFound => StatusCode::UNAUTHORIZED,
            LastFmError(_) => StatusCode::BAD_REQUEST,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

pub async fn search(
    Path(room_id): Path<RoomID>,
    Query(request): Query<SearchRequest>,
    user: User,
    State(state): State<AppState>,
) -> Result<Json<SearchResult>, SearchError> {
    if (Role::User { room_id }) != user.role {
        return Err(SearchError::UserNotInRoom);
    }
    let result = match state.client.search(&request.query).await {
        LastFmResult::Ok { results } => results,
        LastFmResult::Err(e) => return Err(SearchError::LastFmError(e.to_string())),
    };

    let mut rooms = ROOMS.write().map_err(|_| SearchError::InternalError)?;
    let room = rooms
        .iter_mut()
        .find(|r| r.id == room_id)
        .ok_or(SearchError::RoomNotFound)?;

    let tracks = result.into_tracs();
    let mut musics = Vec::with_capacity(tracks.len());
    for music in tracks {
        let full_name = format!("{} - {}", music.artist, music.name);
        let entry = room.musics_to_id.entry(full_name).or_insert_with(|| {
            let id = room.musics.len();
            room.musics.push(Music {
                title: music.name.to_owned(),
                artist: music.artist.to_owned(),
            });
            id
        });
        musics.push(SearchMusic {
            title: music.name,
            artist: music.artist,
            id: *entry,
        });
    }

    Ok(Json(SearchResult { musics }))
}
