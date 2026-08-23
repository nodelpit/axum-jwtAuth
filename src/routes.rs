use crate::{
    auth::{authorize_middleware, sign_in},
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
            get(hello).layer(middleware::from_fn_with_state(
                config.clone(),
                authorize_middleware,
            )),
        )
        .with_state(config)
}
