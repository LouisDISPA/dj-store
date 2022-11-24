use axum::http::HeaderValue;
use tower_http::cors::{AllowOrigin, Any};

pub fn parse_allow_origin(var: &str) -> AllowOrigin {
    if var.trim() == "*" {
        Any.into()
    } else {
        let origins: Vec<HeaderValue> = var.split(',').flat_map(str::parse).collect();
        origins.into()
    }
}
