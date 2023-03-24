// #[cfg(not(feature = "embed-ui"))]
pub mod cors;
pub mod jwt;
// pub mod lastfm;
#[cfg(feature = "https")]
pub mod https;
pub mod room_id;

use std::env;

pub fn required_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(_) => panic!("Missing {} env var", key),
    }
}
