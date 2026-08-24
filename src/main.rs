// main.rs
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::sync::Arc;
use tokio::net::TcpListener;

use axum_jwt_auth::{
    app,
    config::Config,
    state::{AppState, AppStateInner},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    // Lecture de l'environnement (chaînes brutes)
    let config = Config::from_env()?;

    // Lecture + parsing des clés UNE seule fois, au démarrage
    let private_pem = std::fs::read(&config.jwt_private_key)?;
    let public_pem = std::fs::read(&config.jwt_public_key)?;

    let state = AppState {
        inner: Arc::new(AppStateInner {
            encoding_key: EncodingKey::from_rsa_pem(&private_pem)?,
            decoding_key: DecodingKey::from_rsa_pem(&public_pem)?,
            auth_email: config.auth_email,
            auth_first_name: config.auth_first_name,
            auth_last_name: config.auth_last_name,
            auth_password_hash: config.auth_password_hash,
        }),
    };

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app(state)).await?;
    Ok(())
}
