use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}
