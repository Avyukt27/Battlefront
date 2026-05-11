use axum::{Router, routing::post};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;

use crate::{
    game::GameState,
    routes::{move_player_handler, roll_dice_handler},
};

mod game;
mod routes;

struct AppState {
    game: Mutex<GameState>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        game: Mutex::new(GameState::new(8, 8)),
    });

    let app = Router::new()
        .route("/api/roll", post(roll_dice_handler))
        .route("/api/move", post(move_player_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
