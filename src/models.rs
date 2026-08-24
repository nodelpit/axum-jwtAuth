use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
