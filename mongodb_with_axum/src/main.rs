mod handlers;
mod model;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{create_user, get_users};
use mongodb::{Client, options::ClientOptions};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // MongoDB URI
    let uri = "mongodb://localhost:27017";

    // Parse options
    let client_options = ClientOptions::parse(uri).await?;

    // Create client
    let client = Client::with_options(client_options)?;

    // Select database
    let db = client.database("axum_mongo_db");

    // Shared state
    let app_state = AppState { db };

    // Build router
    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .with_state(app_state);

    println!("Server running on http://localhost:3000");

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listner, app).await.unwrap();
    Ok(())
}
