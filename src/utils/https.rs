use std::env;
use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use log::info;

pub async fn run_https_server(addr: SocketAddr, app: Router) {
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
