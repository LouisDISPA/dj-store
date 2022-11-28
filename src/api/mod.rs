use std::{env, sync::Arc};

use crate::utils::lastfm::Client;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

mod admin;
mod room;
pub mod room_id;
mod search;

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
        .route(
            "/admin/rooms",
            get(admin::get_rooms).post(admin::create_room),
        )
        .route("/room/:room/join", get(room::join))
        .route("/room/:room/musics", get(room::get_musics))
        .route("/room/:room/search", get(search::search))
        // .route("/api/room/:room/artist", get(search::get_artist))
        .route("/room/:room/vote", post(room::vote))
        // .route("/api/room/:room/ws", todo!())
        .with_state(state)
}
