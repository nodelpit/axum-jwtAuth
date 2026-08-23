use tokio::net::TcpListener;
mod auth;
mod config;
mod error;
mod routes;
mod services;

use crate::config::Config;
use crate::routes::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let addr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);

    let config = Config {
        jwt_private_key: std::env::var("JWT_PRIVATE_KEY_PATH")?,
        jwt_public_key: std::env::var("JWT_PUBLIC_KEY_PATH")?,
        auth_email: std::env::var("AUTH_EMAIL")?,
        auth_first_name: std::env::var("AUTH_FIRST_NAME")?,
        auth_last_name: std::env::var("AUTH_LAST_NAME")?,
        auth_password_hash: std::env::var("AUTH_PASSWORD_HASH")?,
    };

    axum::serve(listener, app(config)).await?;

    Ok(())
}
