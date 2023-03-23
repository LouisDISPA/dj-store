use std::env;

use axum::Router;
use log::info;
use migration::MigratorTrait;
use sea_orm::Database;
use tokio::signal;

#[cfg(feature = "https")]
use axum_server::tls_rustls::RustlsConfig;

mod api;
#[cfg(feature = "embed-ui")]
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
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

// --- this is just for the https server ---

#[cfg(feature = "https")]
use std::net::SocketAddr;

#[cfg(feature = "https")]
async fn run_https_server(addr: SocketAddr, app: Router) {
    match (env::var("CERT_PATH").ok(), env::var("KEY_PATH").ok()) {
        (Some(cert_path), Some(key_path)) => {
            info!(
                "Using TLS certificate '{}' and key {}'",
                cert_path, key_path
            );

            let config = RustlsConfig::from_pem_file(cert_path, key_path)
                .await
                .expect("Failed to load TLS certificate");

            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .ok();
        }
        (None, None) => {
            info!("No TLS certificate provided, using HTTP");

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .ok();
        }
        (None, Some(_)) => panic!("Missing CERT_PATH env var"),
        (Some(_), None) => panic!("Missing KEY_PATH env var"),
    };
}
