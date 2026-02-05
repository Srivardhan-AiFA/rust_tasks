pub mod todos;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::state::Db;
use todos::*;

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/", get(get_todos))
        .route("/addtodo", post(add_todo))
        .route("/updatetodo/{id}", put(update_todo))
        .route("/deletetodo", delete(delete_todo))
        .with_state(db)
}
