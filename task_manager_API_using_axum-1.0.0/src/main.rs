mod app;
mod errors;
mod middleware;
mod models;
mod routes;
mod state;
mod utils;

use app::create_app;
use state::AppState;

#[tokio::main]
async fn main() {
    tracing::info!("🚀 Starting server");

    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind to address");

    tracing::info!("📡 Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Server crashed");
}
