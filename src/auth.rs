use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

pub async fn sign_in(
    State(config): State<Config>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, StatusCode> {
    let user = match retrieve_user_by_email(&user_data.email, &config) {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.email, &config)?;

    Ok(Json(token))
}

fn retrieve_user_by_email(email: &str, config: &Config) -> Option<CurrentUser> {
    let current_user: CurrentUser = CurrentUser {
        email: config.auth_email.clone(),
        first_name: config.auth_first_name.clone(),
        last_name: config.auth_last_name.clone(),
        password_hash: config.auth_password_hash.clone(),
    };
    Some(current_user)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String, config: &Config) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire: TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    let private_key =
        std::fs::read(&config.jwt_private_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    encode(
        &Header::new(Algorithm::RS256),
        &claim,
        &EncodingKey::from_rsa_pem(&private_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn decode_jwt(
    jwt_token: String,
    config: &Config,
) -> Result<TokenData<Claims>, StatusCode> {
    let public_key =
        std::fs::read(&config.jwt_public_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    decode(
        &jwt_token,
        &DecodingKey::from_rsa_pem(&public_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        &Validation::new(Algorithm::RS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn authorize(req: Request, next: Next) -> Result<Response<Body>, ()> {
    Ok(next.run(req).await)
}
