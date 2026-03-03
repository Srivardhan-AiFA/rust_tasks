use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // simple route
    let app = Router::new().route("/", get(|| async { "Hello HTTPS!" }));

    // load cert + key

    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let cert_path = base.join("src/keys/server.pem");
    let key_path = base.join("src/keys/server.key");

    let config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Running on https://{}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
