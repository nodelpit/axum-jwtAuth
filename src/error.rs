use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    MissingToken,
    InvalidToken,
    InvalidCredentials,
    UnknownUser,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                "Missing or malformed Authorization header",
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::UnknownUser => (StatusCode::UNAUTHORIZED, "Unknown user"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, message).into_response()
    }
}
