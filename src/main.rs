use std::env;

use axum::http::Method;
use log::info;
use tower_http::cors::CorsLayer;

mod api;
mod model;
#[cfg(feature = "embed-ui")]
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    model::init();

    let mut app = api::router();

    if let Ok(var) = env::var("CORS_ALLOWED_ORIGINS") {
        let origins = utils::parse_allow_origin(&var);
        let methods = vec![Method::GET, Method::POST];
        info!("CORS allowed origins: {:?}", origins);
        info!("CORS allowed methods: {:?}", methods);

        app = app.layer(
            CorsLayer::new()
                .allow_methods(methods)
                .allow_origin(origins),
        );
    } else {
        info!("CORS disabled");
    }

    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

    // Start listening on the given address.
    let addr = "127.0.0.1:3000".parse().unwrap();
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
