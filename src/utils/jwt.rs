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
    jsonwebtoken::encode(&Header::default(), &Claims::new(user), get_jwt_encoder()).unwrap()
}

pub fn verify(token: &str) -> Result<User, JwtVerifyError> {
    let token_decode = jsonwebtoken::decode(token, get_jwt_decoder(), &Validation::default());
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

// --- JWT secret key management ---

/// A pair of JWT encoder and decoder keys.
/// This is used to avoid having to recompute the keys every time they are used.
/// This is safe because the keys are only initialized once at the start of the program.
struct JwtKeyPair {
    encoder: EncodingKey,
    decoder: DecodingKey,
}

static mut JWT_SECRET: Option<JwtKeyPair> = None;

/// Set the JWT secret. This should only be called once at the start of the program.
///
/// # Panics
///
/// Panics if the secret is empty or if the secret is already set.
pub fn set_jwt_secret(secret: &str) {
    if secret.is_empty() {
        panic!("JWT_SECRET cannot be empty");
    }
    let secret_bytes = secret.as_bytes();

    // This is safe because JWT_SECRET is only initialized once at the start of the program
    unsafe {
        if JWT_SECRET.is_some() {
            panic!("JWT_SECRET is already set");
        }
        JWT_SECRET = Some(JwtKeyPair {
            encoder: EncodingKey::from_secret(secret_bytes),
            decoder: DecodingKey::from_secret(secret_bytes),
        });
    }
}

/// Get the JWT encoder key. This should only be called after the secret is set.
///
/// # Panics
///
/// Panics if the secret is not set.
fn get_jwt_encoder() -> &'static EncodingKey {
    // This is safe because JWT_SECRET is only initialized once at the start of the program
    unsafe { &JWT_SECRET.as_ref().expect("JWT secret not set").encoder }
}

/// Get the JWT decoder key. This should only be called after the secret is set.
///
/// # Panics
///
/// Panics if the secret is not set.
fn get_jwt_decoder() -> &'static DecodingKey {
    // This is safe because JWT_SECRET is only initialized once at the start of the program
    unsafe { &JWT_SECRET.as_ref().expect("JWT secret not set").decoder }
}
