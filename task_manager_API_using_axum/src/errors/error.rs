use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    TaskAlreadyExists,
    TaskNotFound,
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            AppError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists".to_string())
            }
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            AppError::TaskAlreadyExists => {
                (StatusCode::CONFLICT, "Task already exists".to_string())
            }
            AppError::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ErrorResponse { error: message });

        (status, body).into_response()
    }
}
