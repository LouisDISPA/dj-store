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

    Ok(Json(UserToken::new(User {
        uid: Uuid::new_v4(),
        role: Role::Admin,
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoom {
    id: RoomID,
    creation: DateTime<Utc>,
    expiration: DateTime<Utc>,
    user_count: u32,
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

    let rooms = rooms
        .iter()
        .map(|r| GetRoom {
            id: RoomID::new(r.id),
            creation: r.creation_date,
            expiration: r.expiration_date,
            user_count: r.user_count,
        })
        .collect();

    Ok(Json(rooms))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoom {
    id: RoomID,
    expiration: DateTime<Utc>,
}

#[derive(Error, Display, Debug)]
pub enum CreateRoomsError {
    /// Unauthorized
    Unauthorized,
    /// Room id already exists and is not expired
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

    // Check if the room public id already exists and is not expired
    let room_exists = room::Entity::find()
        .filter(room::Column::Id.eq(room.id.value()))
        .one(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to check if room exists: {}", e);
            CreateRoomsError::InternalError
        })?
        .is_some();

    if room_exists {
        return Err(CreateRoomsError::RoomIdAlreadyExists);
    }

    // Create the room in the database
    let room_active_model = room::ActiveModel {
        id: Set(room.id.value()),
        expiration_date: Set(room.expiration),
        ..Default::default()
    };
    let room_model = room_active_model
        .save(&state.db)
        .await
        .and_then(room::ActiveModel::try_into_model)
        .map_err(|e| {
            log::error!("Failed to create room: {}", e);
            CreateRoomsError::InternalError
        })?;

    // Generate the view model
    let res = GetRoom {
        id: room.id,
        creation: room_model.creation_date,
        expiration: room_model.expiration_date,
        user_count: room_model.user_count,
    };

    Ok(Json(res))
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

pub async fn delete_room(State(state): State<ApiState>, user: User, Path(room_id): Path<RoomID>) -> Result<(), DeleteRoomsError> {
    if user.role != Role::Admin {
        return Err(DeleteRoomsError::Unauthorized);
    }

    let rows_affected  = room::Entity::delete_by_id(room_id.value())
        .exec(&state.db)
        .await
        .map_err(|e| {
            log::error!("Failed to delete room: {}", e);
            DeleteRoomsError::InternalError
        })?.rows_affected;

    if rows_affected == 0 {
        return Err(DeleteRoomsError::RoomIdDoesNotExist);
    }
    
    Ok(())
}
