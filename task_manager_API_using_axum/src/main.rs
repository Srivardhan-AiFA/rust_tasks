mod app;
mod errors;
mod middleware;
mod models;
mod routes;
mod state;
mod utils;

use app::create_app;
use state::AppState;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // ── Tracing / logging setup ─────────────────────────────
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting server");

    // ── App state & router ─────────────────────────────────
    let state = AppState::new();
    let app = create_app(state);

    // ── TCP listener ────────────────────────────────────────
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind to address");

    tracing::info!("📡 Listening on http://0.0.0.0:3000");

    // ── Start server ────────────────────────────────────────
    axum::serve(listener, app).await.expect("Server crashed");
}
