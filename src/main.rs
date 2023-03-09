use std::env;

use axum::Router;
use log::info;
use sea_orm::Database;

mod api;
#[cfg(feature = "embed-ui")]
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_adress = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");

    musicbrainz_rs::config::set_user_agent(
        "dj=store/0.1.0 (https://gitlab.insa-rouen.fr/ldispa/dj-store)",
    );

    let db = Database::connect(db_adress)
        .await
        .expect("Failed to connect to database");

    let api = api::router(db);

    // #[cfg(not(feature = "embed-ui"))]
    let api = utils::cors::init(api);

    let app = Router::new().nest("/api", api);

    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

    // Start listening on the given address.
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:3000".to_string());
    let addr = addr.parse().unwrap();
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
