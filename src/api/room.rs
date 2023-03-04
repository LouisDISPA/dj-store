use std::{collections::HashMap, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::utils::jwt::{Role, User, UserToken};

use entity::*;
use sea_orm::prelude::*;

use sea_orm::sea_query::Expr;

use crate::utils::room_id::RoomID;

use super::state::ApiState;

#[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinError {
    /// The room does not exist.
    RoomNotFound,
    /// The room is full.
    RoomFull,
    /// Internal error.
    InternalError,
}

impl IntoResponse for JoinError {
    fn into_response(self) -> Response {
        let status = match self {
            JoinError::RoomNotFound => StatusCode::NOT_FOUND,
            JoinError::RoomFull => StatusCode::UNAUTHORIZED,
            JoinError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn join(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
) -> Result<Json<UserToken>, JoinError> {
    // the sleep is to prevent brut force to find a random room to spam
    // is it really useful and effective (maybe?)
    // one second to join is not to long for the user ?
    sleep(Duration::from_secs(1)).await;

    // Check if the room public id already exists
    let room_exists = room::Entity::find()
        .filter(room::Column::PublicId.eq(room_id.value()))
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to check if room exists: {}", e);
            JoinError::InternalError
        })?
        .is_some();

    if !room_exists {
        return Err(JoinError::RoomNotFound);
    }

    let row_affected = room::Entity::update_many()
        .col_expr(
            room::Column::UserCount,
            Expr::add(Expr::col(room::Column::UserCount), 1),
        )
        // The filtering assume that the public id is unique
        .filter(room::Column::PublicId.eq(room_id.value()))
        .filter(room::Column::UserCount.lt(10))
        .exec(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to increment user count: {}", e);
            JoinError::InternalError
        })?
        .rows_affected;

    match row_affected {
        0 => Err(JoinError::RoomFull),
        1 => Ok(Json(User::new_user(room_id).into())),
        _ => {
            log::error!(
                "More than one Room was update ({}): User join -> user_count + 1",
                row_affected
            );
            Err(JoinError::InternalError)
        }
    }
}

// #[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum VoteError {
//     /// The room does not exist.
//     RoomNotFound,
//     /// The user is not in the room.
//     UserNotInRoom,
//     /// The music does not exist.
//     MusicNotFound,
// }

// impl IntoResponse for VoteError {
//     fn into_response(self) -> Response {
//         let status = match self {
//             VoteError::RoomNotFound => StatusCode::NOT_FOUND,
//             VoteError::UserNotInRoom => StatusCode::UNAUTHORIZED,
//             VoteError::MusicNotFound => StatusCode::BAD_REQUEST,
//         };

//         let body = self.to_string();

//         (status, body).into_response()
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub struct VoteBody {
//     music_id: usize,
//     voted: bool,
// }

// pub async fn vote(
//     Path(room_id): Path<RoomID>,
//     user: User,
//     Json(vote): Json<VoteBody>,
// ) -> Result<Json<()>, VoteError> {
//     if (Role::User { room_id }) != user.role && user.role != Role::Admin {
//         return Err(VoteError::UserNotInRoom);
//     }
//     let mut rooms = ROOMS.write().unwrap();
//     let room = rooms
//         .iter_mut()
//         .find(|r| r.id == room_id)
//         .ok_or(VoteError::RoomNotFound)?;

//     let music = room
//         .musics
//         .get_mut(vote.music_id)
//         .ok_or(VoteError::MusicNotFound)?;

//     if vote.voted {
//         music.votes += 1;
//         room.votes.push(Vote {
//             user_id: user.uid,
//             music_id: vote.music_id,
//             datetime: chrono::Utc::now(),
//         });
//     } else {
//         music.votes -= 1;
//         room.votes
//             .retain(|v| v.user_id != user.uid || v.music_id != vote.music_id);
//     }

//     room.channel
//         .send(VoteEvent {
//             music_id: vote.music_id,
//             votes: music.votes,
//         })
//         .ok();

//     Ok(Json(()))
// }

// #[derive(Serialize, Deserialize)]
// pub struct Music {
//     id: usize,
//     title: String,
//     artist: String,
//     votes: usize,
//     is_voted: bool,
// }

// #[derive(Error, Display, Debug)]
// pub enum GetMusicError {
//     /// Room not found.
//     RoomNotFound(RoomID),
//     /// User not in room.
//     UserNotInRoom,
//     /// Internal error.
//     InternalError,
//     /// Music not found.
//     MusicNotFound,
// }

// impl IntoResponse for GetMusicError {
//     fn into_response(self) -> Response {
//         use GetMusicError::*;

//         let status: StatusCode = match self {
//             RoomNotFound(_) => StatusCode::NOT_FOUND,
//             UserNotInRoom => StatusCode::UNAUTHORIZED,
//             InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MusicNotFound => StatusCode::BAD_REQUEST,
//         };

//         (status, self.to_string()).into_response()
//     }
// }

// pub async fn get_musics(
//     Path(room_id): Path<RoomID>,
//     user: User,
// ) -> Result<Json<Vec<Music>>, GetMusicError> {
//     if (Role::User { room_id }) != user.role && user.role != Role::Admin {
//         return Err(GetMusicError::UserNotInRoom);
//     }

//     let rooms = ROOMS.read().map_err(|_| GetMusicError::InternalError)?;

//     let room = rooms
//         .iter()
//         .find(|r| r.id == room_id)
//         .ok_or(GetMusicError::RoomNotFound(room_id))?;

//     let mut music_vote = HashMap::new();
//     for vote in room.votes.iter() {
//         let (count, user_vote) = music_vote
//             .entry(vote.music_id.to_owned())
//             .or_insert((0, false));
//         *count += 1;
//         if vote.user_id == user.uid {
//             *user_vote = true;
//         }
//     }

//     let musics = music_vote
//         .into_iter()
//         .map(|(id, (votes, is_voted))| {
//             let music = room.musics.get(id).ok_or(GetMusicError::InternalError)?;
//             Ok(Music {
//                 id,
//                 title: music.title.clone(),
//                 artist: music.artist.clone(),
//                 votes,
//                 is_voted,
//             })
//         })
//         .collect::<Result<Vec<Music>, GetMusicError>>()?;

//     Ok(Json(musics))
// }

// pub async fn get_music_detail(
//     Path((room_id, music_id)): Path<(RoomID, usize)>,
//     user: User,
// ) -> Result<Json<Music>, GetMusicError> {
//     if (Role::User { room_id }) != user.role && user.role != Role::Admin {
//         return Err(GetMusicError::UserNotInRoom);
//     }

//     let rooms = ROOMS.read().map_err(|_| GetMusicError::InternalError)?;

//     let room = rooms
//         .iter()
//         .find(|r| r.id == room_id)
//         .ok_or(GetMusicError::RoomNotFound(room_id))?;

//     let music = room
//         .musics
//         .get(music_id)
//         .ok_or(GetMusicError::MusicNotFound)?;

//     Ok(Json(Music {
//         id: music_id,
//         title: music.title.clone(),
//         artist: music.artist.clone(),
//         votes: music.votes,
//         is_voted: false,
//     }))
// }
