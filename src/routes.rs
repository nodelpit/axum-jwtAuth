use crate::{
    auth::{authorize, sign_in},
    config::Config,
    services::hello,
};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn app(config: Config) -> Router {
    Router::new()
        .route("/sign_in", post(sign_in))
        .route(
            "/protected/",
            get(hello).layer(middleware::from_fn(authorize)),
        )
        .with_state(config)
}
