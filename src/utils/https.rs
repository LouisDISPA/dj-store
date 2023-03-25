use std::env;
use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;

const CERT_KEY: &str = "CERT_PATH";
const KEY_KEY: &str = "KEY_PATH";

/// Run an HTTPS server with the given address and router.
///
/// If the CERT_PATH and KEY_PATH environment variables are set, the server will use TLS.
/// Otherwise, it will use HTTP.
///
/// # Panics
///
/// * If one of the environment variables is set but not the other.
/// * If the TLS certificate could not be loaded.
pub async fn run_https_server(addr: SocketAddr, app: Router) {
    match (env::var(CERT_KEY).ok(), env::var(KEY_KEY).ok()) {
        (Some(cert_path), Some(key_path)) => {
            log::info!(
                "Using TLS certificate '{}' and private key '{}'",
                cert_path,
                key_path
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
            log::info!("No TLS certificate provided, using HTTP");

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .ok();
        }
        (None, Some(_)) => {
            eprintln!("Missing {CERT_KEY} env var");
            std::process::exit(1);
        }
        (Some(_), None) => {
            eprintln!("Missing {KEY_KEY} env var");
            std::process::exit(1);
        }
    };
}
