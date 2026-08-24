// handlers/user.rs
use axum::Json;

use crate::models::{CurrentUser, UserResponse};

pub async fn hello(user: CurrentUser) -> Json<UserResponse> {
    Json(UserResponse {
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    })
}
