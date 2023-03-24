use std::env;

use axum::Router;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio::signal;
#[cfg(feature = "https")]
use utils::https::run_https_server;
use utils::required_env;

mod api;
#[cfg(feature = "embed-ui")]
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_adress = required_env("DATABASE_URL");
    let jwt_secret = required_env("JWT_SECRET");
    let admin_username = required_env("ADMIN_USERNAME");
    let admin_password = required_env("ADMIN_PASSWORD_HASH");

    utils::jwt::set_jwt_secret(&jwt_secret);
    api::set_admin_info(admin_username, admin_password);

    let db = Database::connect(db_adress)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database");

    let api = api::router(db);
    let api = utils::cors::init(api);
    let app = Router::new().nest("/api", api);
    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

    // Start listening on the given address.
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:3000".to_string())
        .parse()
        .unwrap();

    #[cfg(not(feature = "https"))]
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    #[cfg(feature = "https")]
    let server = run_https_server(addr, app);

    info!("Listening on http://{}", addr);
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Shutting down");
        },
        _ = server => {},
    }
}
