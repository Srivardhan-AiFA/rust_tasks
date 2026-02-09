use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;
use tracing::{debug, info, warn};

use crate::{
    errors::error::AppError,
    models::task::{AddTask, Task},
    state::AppState,
};

pub async fn add_task(
    State(state): State<AppState>,
    Extension(name): Extension<String>,
    Json(payload): Json<AddTask>,
) -> Result<Json<String>, AppError> {
    info!(user = %name, title = %payload.title, "Add task request received");

    let users = state.users.lock().await;
    let user_exists = users.iter().any(|u| u.name == name);

    if !user_exists {
        warn!(user = %name, "Add task failed: user not found");
        return Err(AppError::UserNotFound);
    }

    let mut tasks = state.tasks.lock().await;

    let task_exists = tasks
        .iter()
        .any(|t| t.owner == name && t.title == payload.title);

    if task_exists {
        warn!(
            user = %name,
            title = %payload.title,
            "Add task failed: task already exists"
        );
        return Err(AppError::TaskAlreadyExists);
    }

    let next_id = tasks.last().map(|t| t.id + 1).unwrap_or(1);

    let task = Task {
        id: next_id,
        title: payload.title,
        completed: payload.completed,
        owner: name.clone(),
    };

    tasks.push(task);

    info!(user = %name, task_id = next_id, "Task added successfully");

    Ok(Json("Task added successfully".to_string()))
}

pub async fn get_tasks(
    State(state): State<AppState>,
    Extension(name): Extension<String>,
) -> Result<Json<Vec<Task>>, AppError> {
    info!(user = %name, "Fetching tasks");

    let tasks = state.tasks.lock().await;

    let result: Vec<Task> = tasks
        .iter()
        .filter(|task| task.owner == name)
        .cloned()
        .collect();

    debug!(user = %name, count = result.len(), "Tasks fetched");

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct UpdateTask {
    title: Option<String>,
    completed: Option<bool>,
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<String>, AppError> {
    info!(task_id = id, "Update task request received");

    let mut tasks = state.tasks.lock().await;

    for task in tasks.iter_mut() {
        if task.id == id {
            if let Some(title) = payload.title {
                task.title = title;
            }

            if let Some(completed) = payload.completed {
                task.completed = completed;
            }

            info!(task_id = id, "Task updated successfully");
            return Ok(Json("Task updated successfully".to_string()));
        }
    }

    warn!(task_id = id, "Update task failed: task not found");
    Err(AppError::TaskNotFound)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<Json<String>, AppError> {
    info!(task_id = id, "Delete task request received");

    let mut tasks = state.tasks.lock().await;
    let original_len = tasks.len();

    tasks.retain(|task| task.id != id);

    if tasks.len() == original_len {
        warn!(task_id = id, "Delete task failed: task not found");
        Err(AppError::TaskNotFound)
    } else {
        info!(task_id = id, "Task deleted successfully");
        Ok(Json("Task deleted successfully".to_string()))
    }
}
