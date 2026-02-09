use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::errors::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const JWT_SECRET: &[u8] = b"SECRET_KEY";

pub fn create_jwt(name: &str) -> String {
    debug!(user = %name, "Creating JWT");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: name.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap();

    debug!(user = %name, exp = claims.exp, "JWT created");

    token
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    debug!("Verifying JWT");

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|err| {
        warn!(error = %err, "JWT verification failed");
        AppError::InvalidCredentials
    })?;

    debug!(
        user = %data.claims.sub,
        exp = data.claims.exp,
        "JWT verified successfully"
    );

    Ok(data.claims)
}
