use std::{ops::Deref, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, IntoParams};

use self::state::ApiState;

mod admin;
mod room;
mod search;
mod websocket;

pub mod state;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, IntoParams)]
#[into_params(names("music_id"), parameter_in = Path)]
pub struct MusicId(i64);

impl Deref for MusicId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for MusicId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<MusicId> for i64 {
    fn from(id: MusicId) -> Self {
        id.0
    }
}

pub fn router(state: ApiState) -> Router {
    // Limit admin login to 1 request at a time
    // This is ok because for now we only have one admin
    let admin_login = post(admin::login).layer(ConcurrencyLimitLayer::new(1));
    let room_join = get(room::join).layer(ConcurrencyLimitLayer::new(10));

    let handle_error = |err| async move {
        log::error!("Unhandled error: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled error")
    };

    // Deezer API rate limit is 50 requests per 5 seconds
    let rate_limit = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .buffer(128)
        .rate_limit(40, Duration::from_secs(5));

    Router::<ApiState>::new()
        .layer(TraceLayer::new_for_http())
        .route("/admin/login", admin_login)
        .route("/room/all", get(admin::get_rooms))
        .route("/room", post(admin::create_room))
        .route("/room/:room", delete(admin::delete_room))
        .route("/room/:room/join", room_join)
        .route("/room/:room/music/all", get(room::get_musics))
        .route("/room/:room/music/voted", get(room::get_voted_musics))
        .route("/room/:room/music/:music", get(room::get_music_detail))
        .route("/room/:room/vote", post(room::vote))
        .route("/room/:room/search", get(search::search).layer(rate_limit))
        .route("/room/:room/ws", get(websocket::handle_request))
        .with_state(state)
        .fallback(api_fallback)
}

async fn api_fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

#[derive(OpenApi)]
#[openapi(
    paths(
        room::join, room::vote, room::get_musics, room::get_music_detail,
    ),
    components(
        schemas(
            crate::utils::jwt::UserToken, room::VoteBody, room::Music
        )
    ),
    tags(
        (name = "DJ-Store API", description = "API to manage DJ-Store rooms")
    )
)]
pub struct ApiDoc;
