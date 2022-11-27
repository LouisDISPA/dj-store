use axum::Router;
use log::info;

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

    let app = Router::new().nest("/api", api::router());

    #[cfg(not(feature = "embed-ui"))]
    let app = utils::cors::init(app);

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
