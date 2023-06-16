use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;

use self::state::ApiState;

mod admin;
mod room;
mod search;
mod websocket;

pub mod state;

pub type MusicId = i64;

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
