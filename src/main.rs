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

    let api = api::router();

    // #[cfg(not(feature = "embed-ui"))]
    let api = utils::cors::init(api);

    let app = Router::new().nest("/api", api);

    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

    // Start listening on the given address.
    let addr = "0.0.0.0:3000".parse().unwrap();
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
