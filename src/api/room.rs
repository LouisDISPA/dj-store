use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::ROOMS;

use super::room_id::{RoomID, RoomParseError};

pub async fn join(Path(room_id): Path<RoomID>) -> impl IntoResponse {
    todo!()
}

#[derive(Serialize, Deserialize)]
pub struct Music {
    id: usize,
    title: String,
    artist: String,
    votes: usize,
}

#[derive(Error, Display, Debug)]
pub enum GetMusicError {
    /// {0}
    ParsingError(#[from] RoomParseError),
    /// Room not found.
    RoomNotFound(RoomID),
    /// Internal error.
    InternalError,
}

impl IntoResponse for GetMusicError {
    fn into_response(self) -> Response {
        use GetMusicError::*;

        let status: StatusCode = match self {
            ParsingError(_) => StatusCode::BAD_REQUEST,
            RoomNotFound(_) => StatusCode::NOT_FOUND,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

pub async fn get_musics(Path(room): Path<String>) -> Result<Json<Vec<Music>>, GetMusicError> {
    let rooms = ROOMS.read().map_err(|_| GetMusicError::InternalError)?;
    let room_id: RoomID = room.parse()?;

    let room = rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or(GetMusicError::RoomNotFound(room_id))?;

    let mut music_vote = HashMap::new();
    for vote in room.votes.iter() {
        let count = music_vote.entry(vote.music_id).or_insert(0);
        *count += 1;
    }

    let musics = music_vote
        .into_iter()
        .map(|(id, votes)| {
            let music = room.musics.iter().find(|m| m.id == id).unwrap();
            Music {
                id: music.id,
                title: music.title.clone(),
                artist: music.artist.clone(),
                votes,
            }
        })
        .collect();

    Ok(Json(musics))
}
