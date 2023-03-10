use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use migration::SqliteQueryBuilder;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::utils::{
    get_music_or_store_music,
    jwt::{Role, User, UserToken},
};

use entity::*;
use sea_orm::{
    prelude::*, DatabaseBackend, FromQueryResult, QuerySelect, QueryTrait, Set, Statement,
};

use sea_orm::sea_query::{Alias, Expr, Query};

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
    music_id: Uuid,
    voted: bool,
}

impl VoteBody {
    fn to_active_model(&self, user_token: Uuid, room_id: u32) -> vote::ActiveModel {
        vote::ActiveModel {
            user_token: Set(user_token),
            room_id: Set(room_id),
            music_id: Set(self.music_id),
            like: Set(self.voted),
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

    let music = get_music_or_store_music(&state.db, vote.music_id)
        .await
        .map_err(|e| {
            log::error!("Failed to get music: {}", e);
            VoteError::InternalError
        })?;

    let last_vote = vote::Entity::find()
        .column_as(vote::Column::VoteDate.max(), vote::Column::VoteDate)
        .filter(vote::Column::MusicId.eq(music.mbid))
        .filter(vote::Column::UserToken.eq(user.uid))
        .group_by(vote::Column::UserToken)
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to query last vote: {}", e);
            VoteError::InternalError
        })?
        .map(|model| model.like);

    if last_vote == Some(vote.voted) {
        return Err(VoteError::AlreadyVoted);
    }

    vote.to_active_model(user.uid, room_id.value())
        .save(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to save vote: {}", e);
            VoteError::InternalError
        })?;

    // TODO: send channel update

    Ok(())
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
pub struct Music {
    id: Uuid,
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

    // TODO: Use ORM to replace this raw sql if possible.
    let musics = Music::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "SELECT
                    m.'mbid' AS 'id',
                    m.'title',
                    m.'artist',
                    COUNT(v.'music_id') AS 'votes'
                FROM
                    (SELECT
                        MAX('vote'.'vote_date') AS 'vote_date',
                        'vote'.'music_id'
                      FROM
                        'vote'
                        LEFT JOIN 'music' ON 'vote'.'music_id' = 'music'.'mbid'
                      WHERE
                        'vote'.'room_id' = ?
                      GROUP BY
                        'vote'.'music_id',
                        'vote'.'user_token') v,
                    'music' m
                WHERE
                    v.'music_id' = m.'mbid'
                GROUP BY
                    v.'music_id'
                ORDER BY
                    'votes' DESC
                ",
        [room_id.value().into()],
    ))
    .all(&state.db)
    .await
    .map_err(|e| {
        log::error!("Failed to get musics: {}", e);
        GetMusicError::InternalError
    })?;

    Ok(Json(musics))
}

pub async fn get_music_detail(
    State(state): State<ApiState>,
    Path((room_id, music_id)): Path<(RoomID, Uuid)>,
    user: User,
) -> Result<Json<Music>, GetMusicError> {
    if (Role::User { room_id }) != user.role && user.role != Role::Admin {
        return Err(GetMusicError::UserNotInRoom);
    }

    // TODO: Use ORM to replace this raw sql if possible.
    // let music = Music::find_by_statement(Statement::from_sql_and_values(
    //     DatabaseBackend::Sqlite,
    //     "SELECT
    //                 m.'mbid' AS 'id',
    //                 m.'title',
    //                 m.'artist',
    //                 COUNT(v.'user_token') AS 'votes'
    //             FROM
    //                 (SELECT
    //                     MAX('vote'.'vote_date') AS 'vote_date',
    //                     'vote'.'user_token'
    //                 FROM
    //                     'vote'
    //                     LEFT JOIN 'music' ON 'vote'.'music_id' = 'music'.'mbid'
    //                 WHERE
    //                     'vote'.'room_id' = ? AND
    //                     'vote'.'music_id' = ?
    //                 GROUP BY
    //                     'vote'.'user_token'
    //                 ) v,
    //                 'music' m
    //             WHERE
    //                 m.'mbid' = ?
    //             ",
    //     [room_id.value().into(), music_id.into(), music_id.into()],
    // ))
    // .one(&state.db)
    // .await
    // .map_err(|e| {
    //     log::error!("Failed to get musics: {}", e);
    //     GetMusicError::InternalError
    // })?;

    let all_votes = vote::Entity::find()
        .select_only()
        .column_as(vote::Column::VoteDate.max(), "vote_date")
        .column(vote::Column::Like)
        .filter(vote::Column::MusicId.eq(music_id))
        .filter(vote::Column::RoomId.eq(room_id.value()))
        .left_join(music::Entity)
        .group_by(vote::Column::UserToken)
        .into_query();

    let vote_alias = Alias::new("v");

    let (sql, values) = Query::select()
        .columns([music::Column::Title, music::Column::Artist])
        .expr_as(Expr::col(music::Column::Mbid), Alias::new("id"))
        .expr_as(
            Expr::col((vote_alias.clone(), vote::Column::Like)).sum(),
            Alias::new("votes"),
        )
        .from_subquery(all_votes, vote_alias.clone())
        .from(music::Entity)
        .and_where(music::Column::Mbid.eq(music_id))
        .build(SqliteQueryBuilder);

    let music = Music::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        &sql,
        values,
    ))
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
