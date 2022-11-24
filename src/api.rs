use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

use crate::model::{RoomID, ROOMS};

pub fn router() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/api/:room/musics", get(get_musics))
}

#[derive(Serialize, Deserialize)]
struct Music {
    id: usize,
    title: String,
    artist: String,
    votes: usize,
}

#[derive(Serialize, Deserialize)]
enum GetMusicError {
    NotFound,
    InternalError,
}

impl IntoResponse for GetMusicError {
    fn into_response(self) -> Response {
        match self {
            GetMusicError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            GetMusicError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error"),
        }
        .into_response()
    }
}

async fn get_musics(Path(room): Path<String>) -> Result<Json<Vec<Music>>, GetMusicError> {
    let rooms = ROOMS.read().map_err(|_| GetMusicError::InternalError)?;
    let room_id: RoomID = room.parse().map_err(|_| GetMusicError::InternalError)?;

    let room = rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or(GetMusicError::NotFound)?;
    let mut music_vote = HashMap::new();

    for vote in room.votes.iter() {
        let count = music_vote.entry(vote.music_id).or_insert(0);
        *count += 1;
    }

    let musics = music_vote
        .into_iter()
        .map(|(id, votes)| {
            let music = room.musics.iter().find(|m| m.id == id).unwrap();
            Music {
                id: music.id,
                title: music.title.clone(),
                artist: music.artist.clone(),
                votes,
            }
        })
        .collect();

    Ok(Json(musics))
}
