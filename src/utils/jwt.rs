use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    RequestPartsExt, TypedHeader,
};
use chrono::{Duration, Utc};
use displaydoc::Display;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::room_id::RoomID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum Role {
    Admin,
    User { room_id: RoomID },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct User {
    pub uid: Uuid,
    #[serde(flatten)]
    pub role: Role,
}

impl User {
    pub fn new_user(room_id: RoomID) -> Self {
        Self {
            uid: Uuid::new_v4(),
            role: Role::User { room_id },
        }
    }

    pub fn new_admin() -> Self {
        Self {
            uid: Uuid::new_v4(),
            role: Role::Admin,
        }
    }
}

// TODO: Better secret
static JWT_SECRET: &str = "secret";

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    access_token: String,
    token_type: &'static str,
}

impl From<User> for UserToken {
    fn from(user: User) -> Self {
        let token = sign(user);
        Self {
            access_token: token,
            token_type: "Bearer",
        }
    }
}

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        const ERROR: StatusCode = StatusCode::UNAUTHORIZED;

        let auth: TypedHeader<Authorization<Bearer>> = parts.extract().await.map_err(|e| {
            log::error!("Missing authorization header: {}", e);
            ERROR
        })?;

        match verify(auth.token().trim()) {
            Ok(user) => Ok(user),
            Err(err) => {
                log::warn!("{}", err);
                Err(ERROR)
            }
        }
    }
}

#[derive(Debug, Error, Display)]
pub enum JwtVerifyError {
    /// The JWT is invalid: {0}
    InvalidJwt(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    #[serde(flatten)]
    pub user: User,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user: User) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            user,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(user: User) -> String {
    jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(user),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .unwrap()
}

pub fn verify(token: &str) -> Result<User, JwtVerifyError> {
    let token_decode = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    );
    let claim: Claims = match token_decode {
        Ok(token_data) => token_data.claims,
        Err(e) => return Err(JwtVerifyError::InvalidJwt(e.to_string())),
    };
    let now = Utc::now().timestamp();
    if claim.iat > now {
        return Err(JwtVerifyError::InvalidJwt(
            "Token is not valid yet".to_string(),
        ));
    }
    Ok(claim.user)
}
