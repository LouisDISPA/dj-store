use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

mod admin;
mod room;
pub mod room_id;
mod search;

pub fn router() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/api/admin/login", post(admin::login))
        // .route("/admin/rooms", get(admin::get_rooms))
        // .route("/admin/rooms", post(admin::post_room))
        .route("/api/room/:room/join", get(room::join))
        .route("/api/room/:room/musics", get(room::get_musics))
        .route("/api/room/:room/search", get(search::search))
        .route("/api/room/:room/vote", post(room::vote))
    // .route("/api/room/:room/ws", todo!())
}
