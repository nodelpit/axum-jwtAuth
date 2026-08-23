use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{Response, StatusCode, header},
    middleware::Next,
};

use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::AuthError};

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

// The client sends its email and password to /sign_in.
pub async fn sign_in(
    State(config): State<Config>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, StatusCode> {
    // Find the user from simulated user store (.env)
    let user = match retrieve_user_by_email(&user_data.email, &config) {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Compare the submitted password with the stored brcypt hash.
    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // If credentials are valid: Create a signed JWT containing the user's identity.
    let token = encode_jwt(user.email, &config)?;

    // Send the JWT back to the client, he will be able to use it on protected requests.
    Ok(Json(token))
}

// The .env file acts as a fake DB containing a single user.
fn retrieve_user_by_email(email: &str, config: &Config) -> Option<CurrentUser> {
    if email != config.auth_email {
        return None;
    }

    let current_user: CurrentUser = CurrentUser {
        email: config.auth_email.clone(),
        first_name: config.auth_first_name.clone(),
        last_name: config.auth_last_name.clone(),
        password_hash: config.auth_password_hash.clone(),
    };
    Some(current_user)
}

// Verify a plain-text password against a brcypt hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

// Hash a password with brcypt when creating or storing a password
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

// Create the JWT claims, the sign them with the RSA private key.
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

// The client sends the JWT in the Authorization header when accessing a protected route.
pub fn decode_jwt(jwt_token: String, config: &Config) -> Result<TokenData<Claims>, StatusCode> {
    let public_key =
        std::fs::read(&config.jwt_public_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Verify the JWT signature and its claims using the RSA public key.
    decode(
        &jwt_token,
        &DecodingKey::from_rsa_pem(&public_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        &Validation::new(Algorithm::RS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

// The middleware intercepts the protected request before it reaches the handler.
pub async fn authorize_middleware(
    State(config): State<Config>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {

    // Extract the Authorization header and retrieve the Bearer token.
    let auth_header = req.headers_mut().get(header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            });
        }
    };

    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    // Decode and verify the token, an invalid or expired JWT stops the request here.
    let token = token.ok_or_else(|| AuthError {
        message: "Malformed Authorization header".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let token_data = match decode_jwt(token.to_string(), &config) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    // Use the email from the verified JWT to retrieve the corresponding user.
    let current_user = match retrieve_user_by_email(&token_data.claims.email, &config) {
        Some(user) => user,
        None => {
            return Err(AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    // Attach the authenticated user to the request so the protected handler can access it.
    req.extensions_mut().insert(current_user);

    // Authentication succeeded, so let the request continue to the protected handler.
    Ok(next.run(req).await)
}