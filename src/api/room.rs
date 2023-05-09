use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

use crate::utils::jwt::{Role, User, UserToken};

use entity::*;
use sea_orm::{prelude::*, FromQueryResult, JoinType, QuerySelect, QueryTrait, Set};

use sea_orm::sea_query::{Alias, Expr, Order, Query};

use crate::utils::room_id::RoomID;

use super::{search::get_music_or_store_music, state::ApiState, websocket::VoteEvent, MusicId};

#[api_macro::error(internal_error)]
#[default_status(StatusCode::UNAUTHORIZED)]
pub enum JoinError {
    /// The room does not exist
    #[status(StatusCode::NOT_FOUND)]
    RoomNotFound,
    /// The room is full
    RoomFull,
    /// The room is closed
    RoomExpired,
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
    let room = room::Entity::find()
        .filter(room::Column::PublicId.eq(room_id.value()))
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to check if room exists: {}", e);
            JoinError::InternalError
        })?;

    let Some(room) = room else {
        return Err(JoinError::RoomNotFound);
    };

    if room.expiration_date < Utc::now() {
        return Err(JoinError::RoomExpired);
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
        1 => Ok(Json(
            User::new_user(room_id).into_token(room.expiration_date),
        )),
        _ => {
            log::error!(
                "More than one Room was update ({}): User join -> user_count + 1",
                row_affected
            );
            Err(JoinError::InternalError)
        }
    }
}

#[api_macro::error(internal_error, unauthorized)]
pub enum VoteError {
    /// The music does not exist
    #[status(StatusCode::BAD_REQUEST)]
    MusicNotFound,
    /// Already voted for the music.
    #[status(StatusCode::BAD_REQUEST)]
    AlreadyVoted,
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
            music_id: Set(self.music_id),
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
        return Err(VoteError::Unauthorized);
    }

    let music = get_music_or_store_music(&state, vote.music_id)
        .await
        .map_err(|e| {
            log::error!("Failed to get music: {}", e);
            VoteError::MusicNotFound
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

    let event = VoteEvent {
        music_id: music.id,
        like: vote.like,
    };

    state.rooms_channels.send_vote(room_id, event);

    Ok(())
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
pub struct Music {
    id: i64,
    title: String,
    artist: String,
    preview_url: Option<String>,
    image_hash: Option<String>,
    votes: u32,
}

#[api_macro::error(internal_error, unauthorized)]
pub enum GetMusicError {
    /// Music not found
    #[status(StatusCode::BAD_REQUEST)]
    MusicNotFound,
}

pub async fn get_musics(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    user: User,
) -> Result<Json<Vec<Music>>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::Unauthorized);
    }

    let votes = Alias::new("votes");

    let all_votes = vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .column(vote::Column::Like)
        .column(vote::Column::MusicId)
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .group_by(vote::Column::UserToken)
        .group_by(vote::Column::MusicId)
        .into_query();

    let mut statement = Query::select()
        .columns([
            music::Column::Title,
            music::Column::Artist,
            music::Column::Id,
            music::Column::PreviewUrl,
            music::Column::ImageHash,
        ])
        .and_where(music::Column::Id.is_not_null())
        .expr_as(vote::Column::Like.sum(), votes.clone())
        .group_by_col(music::Column::Id)
        .from_subquery(all_votes, vote::Entity)
        .join(
            JoinType::LeftJoin,
            music::Entity,
            Expr::col(vote::Column::MusicId).equals(music::Column::Id),
        )
        .and_having(Expr::col(votes.clone()).gt(0))
        .order_by(votes, Order::Desc)
        .take();

    if user.role != Role::Admin {
        statement.limit(10);
    }

    let backend = state.db.get_database_backend();

    Music::find_by_statement(backend.build(&statement))
        .all(&state.db)
        .await
        .map(Json::from)
        .map_err(|e| {
            log::error!("Failed to get musics: {}", e);
            GetMusicError::InternalError
        })
}

pub async fn get_music_detail(
    State(state): State<ApiState>,
    Path((room_id, music_id)): Path<(RoomID, MusicId)>,
    user: User,
) -> Result<Json<Music>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::Unauthorized);
    }

    let all_votes = vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), "vote_date")
        .column(vote::Column::Like)
        .filter(vote::Column::MusicId.eq(music_id))
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .left_join(music::Entity)
        .group_by(vote::Column::UserToken)
        .into_query();

    let statement = Query::select()
        .columns([
            music::Column::Title,
            music::Column::Artist,
            music::Column::Id,
            music::Column::PreviewUrl,
            music::Column::ImageHash,
        ])
        .expr_as(vote::Column::Like.sum(), Alias::new("votes"))
        .from_subquery(all_votes, vote::Entity)
        .from(music::Entity)
        .and_where(music::Column::Id.eq(music_id))
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
    music_id: i64,
    vote_date: DateTimeUtc,
    like: bool,
}

#[api_macro::error(internal_error, unauthorized)]
pub enum GetVotedMusicError {}

pub async fn get_voted_musics(
    State(state): State<ApiState>,
    Path(room_id): Path<RoomID>,
    user: User,
) -> Result<Json<Vec<VotedMusic>>, GetVotedMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetVotedMusicError::Unauthorized);
    }

    vote::Entity::find()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .filter(vote::Column::UserToken.eq(user.uid))
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .group_by(vote::Column::MusicId)
        .into_model()
        .all(&state.db)
        .await
        .map(Json::from)
        .map_err(|e| {
            log::error!("Failed to get voted musics: {}", e);
            GetVotedMusicError::InternalError
        })
}
