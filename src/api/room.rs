use std::{collections::HashMap, time::Duration};

use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::model::{User, Vote, ROOMS, USERS};

use super::room_id::{RoomID, RoomParseError};

#[derive(Serialize, Deserialize)]
pub struct JoinToken {
    access_token: Uuid,
    token_type: &'static str,
}

#[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinError {
    /// The room ID is invalid: {0}
    InvalidRoomID(#[from] RoomParseError),
    /// The room does not exist.
    RoomNotFound,
}

impl IntoResponse for JoinError {
    fn into_response(self) -> Response {
        let status = match self {
            JoinError::InvalidRoomID(_) => StatusCode::BAD_REQUEST,
            JoinError::RoomNotFound => StatusCode::NOT_FOUND,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn join(Path(room_id): Path<RoomID>) -> Result<Json<JoinToken>, JoinError> {
    sleep(Duration::from_secs(1)).await;
    let rooms = ROOMS.read().unwrap();
    rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or(JoinError::RoomNotFound)?;

    let mut users = USERS.write().unwrap();
    let token = Uuid::new_v4();
    users.push(User { token, room_id });
    Ok(Json(JoinToken {
        access_token: token,
        token_type: "Bearer",
    }))
}

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        const MISSING: (StatusCode, &str) =
            (StatusCode::UNAUTHORIZED, "Missing Authorization header");
        const INVALID: (StatusCode, &str) =
            (StatusCode::UNAUTHORIZED, "Invalid Authorization header");
        const NOT_FOUND: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "User not found");

        let token = parts.headers.remove("Authorization").ok_or(MISSING)?;
        let token = token.to_str().map_err(|_| INVALID)?;
        let token = token.strip_prefix("Bearer ").ok_or(INVALID)?;
        let token = token.parse().map_err(|_| INVALID)?;

        let users = USERS.read().unwrap();
        let user = users.iter().find(|u| u.token == token).ok_or(NOT_FOUND)?;

        return Ok(*user);
    }
}

#[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoteError {
    /// The room does not exist.
    RoomNotFound,
    /// The user is not in the room.
    UserNotInRoom,
    /// The music does not exist.
    MusicNotFound,
}

impl IntoResponse for VoteError {
    fn into_response(self) -> Response {
        let status = match self {
            VoteError::RoomNotFound => StatusCode::NOT_FOUND,
            VoteError::UserNotInRoom => StatusCode::UNAUTHORIZED,
            VoteError::MusicNotFound => StatusCode::BAD_REQUEST,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct VoteBody {
    music_id: usize,
}

pub async fn vote(
    Path(room_id): Path<RoomID>,
    user: User,
    Json(vote): Json<VoteBody>,
) -> Result<Json<()>, VoteError> {
    if room_id != user.room_id {
        return Err(VoteError::UserNotInRoom);
    }
    let mut rooms = ROOMS.write().unwrap();
    let room = rooms
        .iter_mut()
        .find(|r| r.id == room_id)
        .ok_or(VoteError::RoomNotFound)?;

    let music = room
        .musics
        .iter_mut()
        .find(|m| m.id == vote.music_id)
        .ok_or(VoteError::MusicNotFound)?;
    room.votes.push(Vote {
        user_id: user.token,
        music_id: music.id,
        datetime: chrono::Utc::now(),
    });

    Ok(Json(()))
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
    /// The room ID Error: {0}
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
