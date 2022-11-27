use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    model::{Role, USERS},
    utils::jwt::UserToken,
};

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
