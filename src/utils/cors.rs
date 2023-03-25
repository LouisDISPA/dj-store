use std::env;

use axum::{
    http::{header, HeaderValue, Method},
    Router,
};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// Initialize CORS middleware.
///
/// If the CORS_ALLOWED_ORIGINS environment variable is set, the server will use CORS.
pub fn init(app: Router) -> Router {
    if let Ok(var) = env::var("CORS_ALLOWED_ORIGINS") {
        let origins = parse_allow_origin(&var);
        let methods = vec![Method::GET, Method::POST, Method::DELETE];
        let headers = [header::AUTHORIZATION, header::CONTENT_TYPE];

        log::info!("CORS allowed origins: {:?}", origins);
        log::info!("CORS allowed methods: {:?}", methods);
        log::info!("CORS allowed headers: {:?}", headers);

        app.layer(
            CorsLayer::new()
                .allow_methods(methods)
                .allow_origin(origins)
                .allow_headers(headers),
        )
    } else {
        log::info!("CORS disabled");
        app
    }
}

fn parse_allow_origin(var: &str) -> AllowOrigin {
    if var.trim() == "*" {
        Any.into()
    } else {
        let origins: Vec<HeaderValue> = var.split(',').flat_map(str::parse).collect();
        origins.into()
    }
}
