use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String, state: &AppState) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(24)).timestamp() as usize,
        email,
    };
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &state.inner.encoding_key,
    )
    .map_err(|_| AppError::Internal)
}

pub fn decode_jwt(token: &str, state: &AppState) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &state.inner.decoding_key,
        &Validation::new(Algorithm::RS256),
    )
    .map_err(|_| AppError::InvalidToken)
}
