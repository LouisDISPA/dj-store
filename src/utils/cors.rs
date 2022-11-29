use std::env;

use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::{
    http::{HeaderValue, Method},
    Router,
};
use log::info;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub fn init(app: Router) -> Router {
    if let Ok(var) = env::var("CORS_ALLOWED_ORIGINS") {
        let origins = parse_allow_origin(&var);
        let methods = vec![Method::GET, Method::POST];
        let headers = [AUTHORIZATION, CONTENT_TYPE];
        info!("CORS allowed origins: {:?}", origins);
        info!("CORS allowed methods: {:?}", methods);
        info!("CORS allowed headers: {:?}", headers);
        app.layer(
            CorsLayer::new()
                .allow_methods(methods)
                .allow_origin(origins)
                .allow_headers(headers),
        )
    } else {
        info!("CORS disabled");
        app
    }
}

pub fn parse_allow_origin(var: &str) -> AllowOrigin {
    if var.trim() == "*" {
        Any.into()
    } else {
        let origins: Vec<HeaderValue> = var.split(',').flat_map(str::parse).collect();
        origins.into()
    }
}
