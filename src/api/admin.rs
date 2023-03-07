use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use entity::*;
use sea_orm::{prelude::*, ActiveValue::Set, TryIntoModel};

use crate::utils::{
    jwt::{Role, User, UserToken},
    room_id::RoomID,
};

use super::state::ApiState;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[derive(Error, Display, Debug)]
pub enum LoginError {
    /// The username or password is incorrect.
    InvalidCredentials,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        let status = match self {
            LoginError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn login(Json(body): Json<LoginBody>) -> Result<Json<UserToken>, LoginError> {
    let username = body.username;
    let password = body.password;

    // TODO: Check username and password
    if username != "admin" || password != "admin" {
        return Err(LoginError::InvalidCredentials);
    }

    // TODO: Check if user is in database

    Ok(Json(User::new_admin().into()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoom {
    id: RoomID,
    creation: DateTime<Utc>,
    expiration: DateTime<Utc>,
    user_count: u32,
}

impl From<room::Model> for GetRoom {
    fn from(model: room::Model) -> Self {
        Self {
            id: RoomID::new(model.public_id),
            creation: model.creation_date,
            expiration: model.expiration_date,
            user_count: model.user_count,
        }
    }
}

#[derive(Error, Display, Debug)]
pub enum GetRoomsError {
    /// Unauthorized
    Unauthorized,
    /// Internal error
    InternalError,
}

impl IntoResponse for GetRoomsError {
    fn into_response(self) -> Response {
        let status = match self {
            GetRoomsError::Unauthorized => StatusCode::UNAUTHORIZED,
            GetRoomsError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn get_rooms(
    State(state): State<ApiState>,
    user: User,
) -> Result<Json<Vec<GetRoom>>, GetRoomsError> {
    if user.role != Role::Admin {
        return Err(GetRoomsError::Unauthorized);
    }

    let rooms = room::Entity::find().all(&state.db).await.map_err(|e| {
        log::error!("Failed to get rooms: {}", e);
        GetRoomsError::InternalError
    })?;

    let rooms = rooms.into_iter().map(GetRoom::from).collect();

    Ok(Json(rooms))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoom {
    id: RoomID,
    expiration: DateTime<Utc>,
}

impl CreateRoom {
    fn into_active_model(&self) -> room::ActiveModel {
        room::ActiveModel {
            public_id: Set(self.id.value()),
            expiration_date: Set(self.expiration),
            ..Default::default()
        }
    }
}

#[derive(Error, Display, Debug)]
pub enum CreateRoomsError {
    /// Unauthorized
    Unauthorized,
    /// Room id already exists
    RoomIdAlreadyExists,
    /// Internal error
    InternalError,
}

impl IntoResponse for CreateRoomsError {
    fn into_response(self) -> Response {
        let status = match self {
            CreateRoomsError::Unauthorized => StatusCode::UNAUTHORIZED,
            CreateRoomsError::RoomIdAlreadyExists => StatusCode::CONFLICT,
            CreateRoomsError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn create_room(
    State(state): State<ApiState>,
    user: User,
    Json(room): Json<CreateRoom>,
) -> Result<Json<GetRoom>, CreateRoomsError> {
    if user.role != Role::Admin {
        return Err(CreateRoomsError::Unauthorized);
    }

    // Create the room in the database
    match room
        .into_active_model()
        .save(&state.db)
        .await
        .and_then(room::ActiveModel::try_into_model)
        .map(GetRoom::from) 
    {
        Ok(room) => Ok(Json(room)),
        Err(DbErr::Exec(RuntimeErr::SqlxError(err)))
            // Ugly line to get the error code from sqlx and check if it's a duplicate key error
            if err.as_database_error()
                .and_then(|e| e.code())
                .as_deref() == Some("2067") => {
            log::error!("Failed to create room '{}': already exist", err);
            Err(CreateRoomsError::RoomIdAlreadyExists)
        }
        Err(e) => {
            log::error!("Failed to create room '{}': {}", room.id, e);
            Err(CreateRoomsError::InternalError)
        }
    }
}

#[derive(Error, Display, Debug)]
pub enum DeleteRoomsError {
    /// Unauthorized
    Unauthorized,
    /// Room id does not exist
    RoomIdDoesNotExist,
    /// Internal error
    InternalError,
}

impl IntoResponse for DeleteRoomsError {
    fn into_response(self) -> Response {
        let status = match self {
            DeleteRoomsError::Unauthorized => StatusCode::UNAUTHORIZED,
            DeleteRoomsError::RoomIdDoesNotExist => StatusCode::NOT_FOUND,
            DeleteRoomsError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();

        (status, body).into_response()
    }
}

pub async fn delete_room(
    State(state): State<ApiState>,
    user: User,
    Path(room_id): Path<RoomID>,
) -> Result<(), DeleteRoomsError> {
    if user.role != Role::Admin {
        return Err(DeleteRoomsError::Unauthorized);
    }

    let rows_affected = room::Entity::delete_by_id(room_id.value())
        .exec(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to delete room: {}", e);
            DeleteRoomsError::InternalError
        })?
        .rows_affected;

    if rows_affected == 0 {
        return Err(DeleteRoomsError::RoomIdDoesNotExist);
    }

    Ok(())
}
