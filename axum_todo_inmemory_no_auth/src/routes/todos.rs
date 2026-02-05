use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::models::todo::{AddTodo, DeleteTodo, Todo, UpdateTodo};
use crate::state::Db;

// GET /
pub async fn get_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let db = db.lock().await;
    Json(db.clone())
}

// POST /addtodo
pub async fn add_todo(State(db): State<Db>, Json(payload): Json<AddTodo>) -> Json<String> {
    let mut db = db.lock().await;
    let id = db.len() as u32 + 1;

    db.push(Todo {
        id,
        title: payload.title,
        completed: payload.completed,
    });

    Json(format!("Todo {} added successfully", id))
}

// PUT /updatetodo/{id}
pub async fn update_todo(
    State(db): State<Db>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Json<String> {
    let mut db = db.lock().await;

    if let Some(todo) = db.iter_mut().find(|t| t.id == id) {
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }
        return Json(format!("Todo {} updated successfully", id));
    }

    Json(format!("Todo {} not found", id))
}

// DELETE /deletetodo?id=1
pub async fn delete_todo(State(db): State<Db>, Query(params): Query<DeleteTodo>) -> Json<String> {
    let mut db = db.lock().await;
    let before = db.len();

    db.retain(|todo| todo.id != params.id);

    if db.len() == before {
        return Json(format!("Todo {} not found", params.id));
    }

    Json(format!("Todo {} deleted successfully", params.id))
}
