use std::time::Duration;

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
use sea_orm::{prelude::*, FromQueryResult, JoinType, QuerySelect, QueryTrait, Set};

use sea_orm::sea_query::{Alias, Expr, Order, Query};

use crate::utils::room_id::RoomID;

use super::{search::get_music_or_store_music, state::ApiState, websocket::VoteEvent, MusicId};

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
        .filter(room::Column::UserCount.lt(1000))
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

#[derive(Error, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoteError {
    /// The room does not exist.
    RoomNotFound,
    /// The user is not in the room.
    UserNotInRoom,
    /// The music does not exist.
    MusicNotFound,
    /// Already voted for the music.
    AlreadyVoted,
    /// Internal error.
    InternalError,
}

impl IntoResponse for VoteError {
    fn into_response(self) -> Response {
        let status = match self {
            VoteError::RoomNotFound => StatusCode::NOT_FOUND,
            VoteError::UserNotInRoom => StatusCode::UNAUTHORIZED,
            VoteError::AlreadyVoted => StatusCode::BAD_REQUEST,
            VoteError::MusicNotFound => StatusCode::BAD_REQUEST,
            VoteError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct VoteBody {
    music_id: MusicId,
    like: bool,
}

impl VoteBody {
    fn to_active_model(&self, user_token: Uuid, room_id: u32) -> vote::ActiveModel {
        vote::ActiveModel {
            user_token: Set(user_token),
            room_id: Set(room_id),
            music_id: Set(self.music_id as i64), // TODO: fix this
            like: Set(self.like),
            ..Default::default()
        }
    }
}

pub async fn vote(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    user: User,
    Json(vote): Json<VoteBody>,
) -> Result<(), VoteError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(VoteError::UserNotInRoom);
    }

    let music = get_music_or_store_music(&state, vote.music_id)
        .await
        .map_err(|e| {
            log::error!("Failed to get music: {}", e);
            VoteError::InternalError
        })?;

    let last_vote = vote::Entity::find()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .filter(vote::Column::MusicId.eq(music.id))
        .filter(vote::Column::UserToken.eq(user.uid))
        .group_by(vote::Column::UserToken)
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to query last vote: {}", e);
            VoteError::InternalError
        })?
        .map(|model| model.like);

    if last_vote == Some(vote.like) {
        return Err(VoteError::AlreadyVoted);
    }

    vote.to_active_model(user.uid, room_id.value())
        .save(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to save vote: {}", e);
            VoteError::InternalError
        })?;

    let rooms = state.rooms_channels.read().map_err(|e| {
        log::error!("Failed to get read lock on rooms: {}", e);
        VoteError::InternalError
    })?;

    if let Some(room) = rooms.get(&room_id) {
        room.send(VoteEvent {
            music_id: music.id as u64, // TODO: fix this
            like: vote.like,
        })
        .map_err(|e| {
            log::error!("Failed to send vote to room: {}", e);
            VoteError::InternalError
        })?;
    }

    Ok(())
}

// TODO: Maybe use struct name to generate column name for the query
// Like in the entity crate when deriving Model on a struct
// a column name is generated from the struct field name
#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
pub struct Music {
    id: i64,
    title: String,
    artist: String,
    votes: u32,
}

#[derive(Error, Display, Debug)]
pub enum GetMusicError {
    /// User not in room.
    UserNotInRoom,
    /// Internal error.
    InternalError,
    /// Music not found.
    MusicNotFound,
}

impl IntoResponse for GetMusicError {
    fn into_response(self) -> Response {
        use GetMusicError::*;

        let status: StatusCode = match self {
            UserNotInRoom => StatusCode::UNAUTHORIZED,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MusicNotFound => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}

pub async fn get_musics(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    user: User,
) -> Result<Json<Vec<Music>>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::UserNotInRoom);
    }

    let all_votes = vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .column(vote::Column::Like)
        .column(vote::Column::MusicId)
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .group_by(vote::Column::UserToken)
        .group_by(vote::Column::MusicId)
        .into_query();

    let statement = Query::select()
        .columns([
            music::Column::Title,
            music::Column::Artist,
            music::Column::Id,
        ])
        .expr_as(vote::Column::Like.sum(), Alias::new("votes"))
        .group_by_col(music::Column::Id)
        .from_subquery(all_votes, vote::Entity)
        .join(
            JoinType::LeftJoin,
            music::Entity,
            Expr::col(vote::Column::MusicId).equals(music::Column::Id),
        )
        .and_having(Expr::col(Alias::new("votes")).gt(0))
        .order_by(Alias::new("votes"), Order::Desc)
        .take();

    let backend = state.db.get_database_backend();

    return Music::find_by_statement(backend.build(&statement))
        .all(&state.db)
        .await
        .map(Json::from)
        .map_err(|e| {
            log::error!("Failed to get musics: {}", e);
            GetMusicError::InternalError
        });
}

pub async fn get_music_detail(
    State(state): State<ApiState>,
    Path((room_id, music_id)): Path<(RoomID, MusicId)>,
    user: User,
) -> Result<Json<Music>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::UserNotInRoom);
    }

    let all_votes = vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), "vote_date")
        .column(vote::Column::Like)
        .filter(vote::Column::MusicId.eq(music_id as i64)) // TODO: fix this
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .left_join(music::Entity)
        .group_by(vote::Column::UserToken)
        .into_query();

    let statement = Query::select()
        .columns([
            music::Column::Title,
            music::Column::Artist,
            music::Column::Id,
        ])
        .expr_as(vote::Column::Like.sum(), Alias::new("votes"))
        .from_subquery(all_votes, vote::Entity)
        .from(music::Entity)
        .and_where(music::Column::Id.eq(music_id as i64)) // TODO: fix this
        .take();

    let backend = state.db.get_database_backend();

    let music = Music::find_by_statement(backend.build(&statement))
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to get musics: {}", e);
            GetMusicError::InternalError
        })?;

    match music {
        Some(music) => Ok(Json(music)),
        None => Err(GetMusicError::MusicNotFound),
    }
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
pub struct VotedMusic {
    music_id: MusicId,
    title: String,
    artist: String,
    vote_date: DateTimeUtc,
    like: bool,
}

#[derive(Error, Display, Debug)]
pub enum GetVotedMusicError {
    /// User not in room.
    UserNotInRoom,
    /// Internal error.
    InternalError,
}

impl IntoResponse for GetVotedMusicError {
    fn into_response(self) -> Response {
        use GetVotedMusicError::*;

        let status: StatusCode = match self {
            UserNotInRoom => StatusCode::UNAUTHORIZED,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

pub async fn get_voted_musics(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    user: User,
) -> Result<Json<Vec<VotedMusic>>, GetVotedMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetVotedMusicError::UserNotInRoom);
    }

    vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .column(vote::Column::MusicId)
        .column(vote::Column::Like)
        .column(music::Column::Title)
        .column(music::Column::Artist)
        .filter(vote::Column::UserToken.eq(user.uid))
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .group_by(vote::Column::MusicId)
        .left_join(music::Entity)
        .into_model()
        .all(&state.db)
        .await
        .map(Json::from)
        .map_err(|e| {
            log::error!("Failed to get voted musics: {}", e);
            GetVotedMusicError::InternalError
        })
}
