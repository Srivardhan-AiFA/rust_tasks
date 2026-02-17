use axum::{
    body::Body,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use tracing::{debug, info, warn};

use crate::{errors::error::AppError, utils::jwt::verify_jwt};

pub async fn protected_route(mut req: Request<Body>, next: Next) -> Result<Response, AppError> {
    debug!("Protected route middleware triggered");

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            warn!("Missing Authorization header");
            AppError::InvalidCredentials
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        warn!("Authorization header is not Bearer token");
        AppError::InvalidCredentials
    })?;

    let claims = verify_jwt(token)?;

    info!(user = %claims.sub, "JWT validated successfully");

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
