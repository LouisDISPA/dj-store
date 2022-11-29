use std::{collections::HashMap, time::Duration};

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{
    model::{Role, User, Vote, ROOMS, USERS},
    utils::jwt::UserToken,
};

use super::room_id::RoomID;

#[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinError {
    /// The room does not exist.
    RoomNotFound,
}

impl IntoResponse for JoinError {
    fn into_response(self) -> Response {
        let status = match self {
            JoinError::RoomNotFound => StatusCode::NOT_FOUND,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn join(Path(room_id): Path<RoomID>) -> Result<Json<UserToken>, JoinError> {
    sleep(Duration::from_secs(1)).await;
    let rooms = ROOMS.read().unwrap();
    rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or(JoinError::RoomNotFound)?;

    let mut users = USERS.write().unwrap();
    let uid = Uuid::new_v4();
    let user = User {
        uid,
        role: Role::User { room_id },
    };
    users.push(user);
    Ok(Json(UserToken::new(user)))
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
    voted: bool,
}

pub async fn vote(
    Path(room_id): Path<RoomID>,
    user: User,
    Json(vote): Json<VoteBody>,
) -> Result<Json<()>, VoteError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(VoteError::UserNotInRoom);
    }
    let mut rooms = ROOMS.write().unwrap();
    let room = rooms
        .iter_mut()
        .find(|r| r.id == room_id)
        .ok_or(VoteError::RoomNotFound)?;

    room.musics
        .get(vote.music_id)
        .ok_or(VoteError::MusicNotFound)?;

    if vote.voted {
        room.votes.push(Vote {
            user_id: user.uid,
            music_id: vote.music_id,
            datetime: chrono::Utc::now(),
        });
    } else {
        room.votes
            .retain(|v| v.user_id != user.uid && v.music_id == vote.music_id);
    }

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
    /// Room not found.
    RoomNotFound(RoomID),
    /// User not in room.
    UserNotInRoom,
    /// Internal error.
    InternalError,
}

impl IntoResponse for GetMusicError {
    fn into_response(self) -> Response {
        use GetMusicError::*;

        let status: StatusCode = match self {
            RoomNotFound(_) => StatusCode::NOT_FOUND,
            UserNotInRoom => StatusCode::UNAUTHORIZED,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

pub async fn get_musics(
    Path(room_id): Path<RoomID>,
    user: User,
) -> Result<Json<Vec<Music>>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::UserNotInRoom);
    }

    let rooms = ROOMS.read().map_err(|_| GetMusicError::InternalError)?;

    let room = rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or(GetMusicError::RoomNotFound(room_id))?;

    let mut music_vote = HashMap::new();
    for vote in room.votes.iter() {
        let count = music_vote.entry(vote.music_id.to_owned()).or_insert(0);
        *count += 1;
    }

    let musics = music_vote
        .into_iter()
        .map(|(id, votes)| {
            let music = room.musics.get(id).ok_or(GetMusicError::InternalError)?;
            Ok(Music {
                id,
                title: music.title.clone(),
                artist: music.artist.clone(),
                votes,
            })
        })
        .collect::<Result<Vec<Music>, GetMusicError>>()?;

    Ok(Json(musics))
}
