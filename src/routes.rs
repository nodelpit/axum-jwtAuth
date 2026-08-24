use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{auth::sign_in, user::hello},
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/sign_in", post(sign_in))
        .route("/protected/", get(hello))
        .with_state(state)
}
