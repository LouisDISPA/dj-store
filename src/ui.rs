use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;

pub fn mount(app: Router) -> Router {
    app.route("/_app/*file", get(static_handler))
        .route("/favicon.png", get(favicon_handler))
        .fallback(index_handler)
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
