use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::create_app;
use crate::models::todo::Todo;
use crate::state::Db;

mod app;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(vec![
        Todo {
            id: 1,
            title: "Go to GYM".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Buy Eggs".to_string(),
            completed: true,
        },
        Todo {
            id: 3,
            title: "Go to Market".to_string(),
            completed: false,
        },
    ]));

    let app = create_app(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
