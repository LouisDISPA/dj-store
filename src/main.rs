use axum::{
    body::{boxed, Full},
    extract::Path,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
    Json,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define our app routes, including a fallback option for anything not matched.

    let api_router = Router::new().route("/:room/votes", get(get_votes));

    let app = Router::new()
        .route("/_app/*file", get(static_handler))
        .route("/favicon.png", get(favicon_handler))
        .fallback(get(index_handler))
        .nest("/api", api_router);

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

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    asset("index.html")
}

async fn favicon_handler() -> impl IntoResponse {
    asset("favicon.png")
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    asset(uri.path().trim_start_matches('/'))
}

#[derive(RustEmbed)]
#[folder = "ui/build/"]
struct Asset;

fn asset(path: &str) -> Response {
    match Asset::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(boxed(Full::from("404 whaat")))
            .unwrap(),
    }
}
