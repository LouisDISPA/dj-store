use std::env;

use axum::Router;
use log::info;
use migration::MigratorTrait;
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
    let jwt_secret = env::var("JWT_SECRET").expect("Missing JWT_SECRET env var");
    let admin_username = env::var("ADMIN_USERNAME").expect("Missing ADMIN_USERNAME env var");
    let admin_password =
        env::var("ADMIN_PASSWORD_HASH").expect("Missing ADMIN_PASSWORD_HASH env var");

    utils::jwt::set_jwt_secret(&jwt_secret);
    api::set_admin_info(admin_username, admin_password);

    let db = Database::connect(db_adress)
        .await
        .expect("Failed to connect to database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database");

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
