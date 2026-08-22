use axum::{body::Body, extract::Request, http::Response, middleware::Next};

pub async fn sign_in() {}

#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub _password_hash: String,
}

pub async fn authorize(req: Request, next: Next) -> Result<Response<Body>, ()> {
    Ok(next.run(req).await)
}
