use axum::{
    body::{boxed, Full},
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;

pub fn mount(app: Router) -> Router {
    app.route("/", get(index_handler))
        .route("/*path", get(file_handler))
}

async fn index_handler() -> impl IntoResponse {
    asset("index.html")
}

async fn file_handler(Path(mut path): Path<String>) -> impl IntoResponse {
    // if there is no extension, assume html
    if path.rfind('.').is_none() {
        path.push_str(".html");
    }
    asset(&path)
}

#[derive(RustEmbed)]
#[folder = "ui/build/"]
struct Asset;

// TODO: cache control

fn asset(path: &str) -> Response {
    match Asset::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            Response::builder()
                .header(header::CONTENT_TYPE, content.metadata.mimetype())
                .header(header::CACHE_CONTROL, "max-age=86400")
                .body(body)
                .unwrap()
        }
        None => match Asset::get("fallback.html") {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header(header::CONTENT_TYPE, content.metadata.mimetype())
                    .header(header::CACHE_CONTROL, "max-age=86400")
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404 whaat")))
                .unwrap(),
        },
    }
}
