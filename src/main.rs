use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio::signal;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{state::ApiState, ApiDoc};

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
    let addr = utils::get_addr();


    // JWT secret should be in the state ?
    utils::jwt::set_jwt_secret(&jwt_secret);

    let db = Database::connect(db_adress)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database");

    let state = ApiState::new(db, admin_username, admin_password);

    let api = api::router(state);
    let api = api.layer(TraceLayer::new_for_http());
    let api = utils::cors::init(api);
    let app = Router::new().nest("/api", api);
    let app =
        app.merge(SwaggerUi::new("/api/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()));
    #[cfg(feature = "embed-ui")]
    let app = ui::mount(app);

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
