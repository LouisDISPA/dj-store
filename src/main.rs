use axum::{
    body::{boxed, Full},
    extract::{Path, rejection::PathRejection},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define our app routes, including a fallback option for anything not matched.

    let api_router = Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route(
            "/:test",
            get(|test: Result<Path<u8>, PathRejection>| async move { 
                match test {
                    Err(PathRejection::FailedToDeserializePathParams(err)) => panic!("Error {}", err.into_kind()),
                    _ => format!("Hello, {:?}", test),
                }
            }),
        );

    let app = Router::new()
        .route("/_app/:file", get(static_handler))
        .route("/favicon.png", get(favicon_handler))
        .fallback(get(index_handler))
        .nest("/api", api_router);

    // Start listening on the given address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    StaticFile("index.html")
}

async fn favicon_handler() -> impl IntoResponse {
    StaticFile("favicon.png")
}

async fn static_handler(Path(file): Path<String>) -> impl IntoResponse {
    println!("file {}", file);
    StaticFile(format!("_app/{}", file))
}

#[derive(RustEmbed)]
#[folder = "ui/build/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: AsRef<str>,
{
    fn into_response(self) -> Response {
        let path = self.0.as_ref();
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
}
