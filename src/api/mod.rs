
use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use self::state::ApiState;

mod admin;
mod room;
// mod search;
// mod websocket;

mod state;

pub fn router(db: DatabaseConnection, api_key: String) -> Router {

    let state = state::ApiState::init(db, api_key);

    Router::<ApiState>::new()
        .layer(TraceLayer::new_for_http())
        .route("/admin/login", post(admin::login))
        .route("/room/all", get(admin::get_rooms))
        .route("/room", post(admin::create_room))
        .route("/room/:room", delete(admin::delete_room))
        .route("/room/:room/join", get(room::join))
        // .route("/room/:room/music/voted", get(room::get_musics))
        // .route("/room/:room/music/:music", get(room::get_music_detail))
        // .route("/room/:room/vote", post(room::vote))
        // .route("/room/:room/search", get(search::search))
        // .route("/api/room/:room/artist", get(search::get_artist))
        // .route("/room/:room/ws", get(websocket::handle_request))
        .with_state(state)
        .fallback(api_fallback)
        
}

async fn api_fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
