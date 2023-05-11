use argon2::{Argon2, PasswordVerifier};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Duration, Utc};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use entity::*;
use sea_orm::{prelude::*, ActiveValue::Set, TryIntoModel};

use crate::utils::{
    jwt::{Role, User, UserToken},
    room_id::RoomID,
};

use super::state::ApiState;

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    username: String,
    password: Secret<String>,
}

#[api_macro::error]
pub enum LoginError {
    /// The username or password is incorrect
    #[status(StatusCode::UNAUTHORIZED)]
    InvalidCredentials,
}

pub async fn login(
    State(state): State<ApiState>,
    Json(body): Json<LoginBody>,
) -> Result<Json<UserToken>, LoginError> {
    let username = body.username;
    let password = body.password;

    let admin = state.admin_info.clone();

    let result = tokio::task::spawn_blocking(move || {
        Argon2::default().verify_password(password.expose_secret().as_bytes(), &admin.password)
    })
    .await
    .map_err(|_| {
        log::error!("Failed to spawn blocking task to verify password");
        LoginError::InvalidCredentials
    })?;

    if state.admin_info.username != username || result.is_err() {
        log::warn!("Invalid credentials for admin login");
        return Err(LoginError::InvalidCredentials);
    }

    log::info!("Admin '{}' logged in", username);

    Ok(Json(
        User::new_admin().into_token(Utc::now() + Duration::days(1)),
    ))
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

#[api_macro::error(internal_error, unauthorized)]
pub enum GetRoomsError {}

pub async fn get_rooms(
    State(state): State<ApiState>,
    user: User,
) -> Result<Json<Vec<GetRoom>>, GetRoomsError> {
    if user.role != Role::Admin {
        return Err(GetRoomsError::Unauthorized);
    }

    let rooms = room::Entity::find().all(&state.db).await?;

    let rooms = rooms.into_iter().map(GetRoom::from).collect();

    Ok(Json(rooms))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoom {
    id: RoomID,
    expiration: DateTime<Utc>,
}

impl CreateRoom {
    fn to_active_model(&self) -> room::ActiveModel {
        room::ActiveModel {
            public_id: Set(self.id.value()),
            expiration_date: Set(self.expiration),
            ..Default::default()
        }
    }
}

#[api_macro::error(internal_error, unauthorized)]
pub enum CreateRoomsError {
    /// Room id already exists
    #[status(StatusCode::CONFLICT)]
    RoomIdAlreadyExists,
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
    room.to_active_model()
        .save(&state.db)
        .await
        .and_then(room::ActiveModel::try_into_model)
        .map(GetRoom::from)
        .map(Json)
        .map_err(|err| {
            // Ugly line to get the error code from sqlx and check if it's a duplicate key error
            if let DbErr::Exec(RuntimeErr::SqlxError(err)) = &err {
                if err.as_database_error().and_then(|e| e.code()).as_deref() == Some("2067") {
                    return CreateRoomsError::RoomIdAlreadyExists;
                }
            }
            CreateRoomsError::InternalError(err)
        })
}

#[api_macro::error(internal_error, unauthorized)]
pub enum DeleteRoomsError {
    /// Room id does not exist
    #[status(StatusCode::NOT_FOUND)]
    RoomIdDoesNotExist,
}

pub async fn delete_room(
    State(state): State<ApiState>,
    user: User,
    Path(room_id): Path<RoomID>,
) -> Result<(), DeleteRoomsError> {
    if user.role != Role::Admin {
        return Err(DeleteRoomsError::Unauthorized);
    }

    let rows_affected = room::Entity::delete_many()
        .filter(room::Column::PublicId.eq(room_id.value()))
        .exec(&state.db)
        .await?
        .rows_affected;

    match rows_affected {
        1 => Ok(()),
        0 => Err(DeleteRoomsError::RoomIdDoesNotExist),
        _ => {
            log::error!(
                "More than one room was deleted: deleted {} rows",
                rows_affected
            );
            Ok(())
        }
    }
}
