use tokio::net::TcpListener;
mod auth;
mod routes;
mod services;

use crate::routes::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let addr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app()).await?;

    Ok(())
}
