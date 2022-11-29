use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    model::{self, Role, User, ROOMS, USERS},
    utils::jwt::UserToken,
};

use super::room_id::RoomID;

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

    let users = USERS.read().unwrap();
    let user = users.iter().find(|u| u.role == Role::Admin).unwrap();
    Ok(Json(UserToken::new(*user)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoom {
    id: RoomID,
    creation: DateTime<Utc>,
    expiration: DateTime<Utc>,
    user_count: usize,
    active: bool,
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

pub async fn get_rooms(user: User) -> Result<Json<Vec<GetRoom>>, GetRoomsError> {
    if user.role != Role::Admin {
        return Err(GetRoomsError::Unauthorized);
    }

    let rooms = ROOMS.read().map_err(|_| GetRoomsError::InternalError)?;
    let users = USERS.read().map_err(|_| GetRoomsError::InternalError)?;
    let rooms = rooms
        .iter()
        .map(|r| GetRoom {
            id: r.id,
            creation: r.creation,
            expiration: r.expiration,
            user_count: users
                .iter()
                .filter(|u| u.role == Role::User { room_id: r.id })
                .count(),
            active: r.active,
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
    user: User,
    Json(room): Json<CreateRoom>,
) -> Result<Json<GetRoom>, CreateRoomsError> {
    if user.role != Role::Admin {
        return Err(CreateRoomsError::Unauthorized);
    }

    let mut rooms = ROOMS.write().map_err(|_| CreateRoomsError::InternalError)?;

    // Check if the room already exists
    if rooms.iter().any(|r| r.id == room.id) {
        return Err(CreateRoomsError::RoomIdAlreadyExists);
    }

    let room = model::Room {
        id: room.id,
        creation: Utc::now(),
        expiration: room.expiration,
        active: true,
        votes: Default::default(),
        musics: Default::default(),
        musics_to_id: Default::default(),
    };

    let res = GetRoom {
        id: room.id,
        creation: room.creation,
        expiration: room.expiration,
        active: room.active,
        user_count: 0,
    };

    rooms.push(room);

    Ok(res.into())
}
