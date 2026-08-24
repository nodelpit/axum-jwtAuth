use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    auth::jwt::decode_jwt, error::AppError, models::CurrentUser, state::AppState,
    store::retrieve_user_by_email,
};

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Récupère le Bearer token — plus de split_whitespace ni d'unwrap
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::MissingToken)?;

        // 2. Décode et vérifie
        let claims = decode_jwt(bearer.token(), state)?.claims;

        // 3. Charge l'utilisateur correspondant
        retrieve_user_by_email(&claims.email, state).ok_or(AppError::UnknownUser)
    }
}
