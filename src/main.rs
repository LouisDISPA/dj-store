use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio::signal;

use crate::api::state::ApiState;

mod api;
#[cfg(feature = "embed-ui")]
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    required_envs!(
        db_adress => "DATABASE_URL"
        jwt_secret => "JWT_SECRET"
        admin_username => "ADMIN_USERNAME"
        admin_password => "ADMIN_PASSWORD_HASH"
    );

    // JWT secret should be in the state
    // just keeping it like this because why not
    utils::jwt::set_jwt_secret(&jwt_secret);

    let db = Database::connect(db_adress)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database");

    let state = ApiState::new(db, admin_username, admin_password);

    let api = api::router(state);
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
    let server = utils::https::run_https_server(addr, app);

    log::info!("Listening on http://{}", addr);
    tokio::select! {
        _ = signal::ctrl_c() => {
            log::info!("Shutting down");
        },
        _ = server => {},
    }
}
