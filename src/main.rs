use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[cfg(feature = "embed-ui")]
mod ui;

#[tokio::main]
async fn main() {
    // Define our app routes, including a fallback option for anything not matched.

    let api_router = Router::new().route("/:room/votes", get(get_votes));
    let app = Router::new().nest("/api", api_router);

    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

    // Start listening on the given address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Votes {
    music: Music,
    count: u32,
}

#[derive(Serialize, Deserialize)]
struct Music {
    id: String,
    name: String,
    artist: String,
}

enum GetVotesError {
    NotFound,
}

impl IntoResponse for GetVotesError {
    fn into_response(self) -> Response {
        match self {
            GetVotesError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
        }
        .into_response()
    }
}

async fn get_votes(Path(room): Path<String>) -> Result<Json<Vec<Votes>>, GetVotesError> {
    if room == "error" {
        return Err(GetVotesError::NotFound);
    }

    Ok(Json(vec![
        Votes {
            music: Music {
                id: "1".to_string(),
                name: "music1".to_string(),
                artist: "artist1".to_string(),
            },
            count: 1,
        },
        Votes {
            music: Music {
                id: "2".to_string(),
                name: "music2".to_string(),
                artist: "artist2".to_string(),
            },
            count: 2,
        },
    ]))
}
