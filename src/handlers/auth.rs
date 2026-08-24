// handlers/auth.rs
use axum::{Json, extract::State};

use crate::{
    auth::{jwt::encode_jwt, password::verify_password},
    error::AppError,
    models::SignInData,
    state::AppState,
    store::retrieve_user_by_email,
};

pub async fn sign_in(
    State(state): State<AppState>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, AppError> {
    let user =
        retrieve_user_by_email(&user_data.email, &state).ok_or(AppError::InvalidCredentials)?;

    if !verify_password(&user_data.password, &user.password_hash).map_err(|_| AppError::Internal)? {
        return Err(AppError::InvalidCredentials);
    }

    let token = encode_jwt(user.email, &state)?;
    Ok(Json(token))
}
