use axum::extract::{Json, State};
use tracing::{info, warn};

use crate::{
    errors::error::AppError,
    models::auth::User,
    state::AppState,
    utils::{
        jwt::create_jwt,
        password::{hash_password, verify_password},
    },
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<String>, AppError> {
    info!(user = %payload.name, "Signup request received");

    let mut users = state.users.lock().await;

    if users.iter().any(|u| u.name == payload.name) {
        warn!(user = %payload.name, "Signup failed: user already exists");
        return Err(AppError::UserAlreadyExists);
    }

    if payload.password.len() < 8 {
        warn!(user = %payload.name, "Signup failed: password too short");
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    users.push(User {
        name: payload.name,
        password: hash_password(&payload.password),
    });

    info!("Signup successful");

    Ok(Json("Signup successful".to_string()))
}

pub async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<String>, AppError> {
    info!(user = %payload.name, "Signin request received");

    let users = state.users.lock().await;

    let user = users
        .iter()
        .find(|u| u.name == payload.name)
        .ok_or_else(|| {
            warn!(user = %payload.name, "Signin failed: user not found");
            AppError::UserNotFound
        })?;

    if !verify_password(&payload.password, &user.password) {
        warn!(user = %payload.name, "Signin failed: invalid credentials");
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(&user.name);

    info!(user = %user.name, "Signin successful");

    Ok(Json(token))
}
