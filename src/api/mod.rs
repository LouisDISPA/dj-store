use std::{env, sync::Arc};

use crate::utils::lastfm::Client;
use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::TraceLayer;

mod admin;
mod room;
pub mod room_id;
mod search;
mod websocket;

#[derive(Debug, Clone)]
pub struct AppState {
    pub client: Arc<Client>,
}

pub fn router() -> Router {
    let state = AppState {
        client: Client::new(&env::var("LASTFM_API_KEY").expect("Missing LASTFM_API_KEY env var"))
            .into(),
    };

    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/admin/login", post(admin::login))
        // .route("/admin/rooms", get(admin::get_rooms))
        .route("/room/all", get(admin::get_rooms))
        .route("/room", post(admin::create_room))
        .route("/room/:room", delete(admin::delete_room))
        .route("/room/:room/join", get(room::join))
        .route("/room/:room/musics", get(room::get_musics))
        .route("/room/:room/search", get(search::search))
        // .route("/api/room/:room/artist", get(search::get_artist))
        .route("/room/:room/vote", post(room::vote))
        .route("/room/:room/ws", get(websocket::handle_request))
        .fallback(api_fallback)
        .with_state(state)
}

async fn api_fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
