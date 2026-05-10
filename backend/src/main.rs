use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::net::SocketAddr;

mod game;

#[derive(Serialize)]
struct Status {
    message: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Status> {
    Json(Status {
        message: "Battlefront Game Online".to_string(),
        status: "OK".to_string(),
    })
}
