use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

// mod admin;
mod room;
pub mod room_id;
mod search;

pub fn router() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        // .route("/admin/login", post(admin::login))
        // .route("/admin/rooms", get(admin::get_rooms))
        // .route("/admin/rooms", post(admin::post_room))
        .route("/room/:room/join", get(room::join))
        .route("/room/:room/musics", get(room::get_musics))
        .route("/room/:room/search", get(search::search))
    // .route("/room/:room/vote", post(room::vote))
}
