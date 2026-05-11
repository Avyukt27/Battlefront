use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;

use crate::game::GameState;

mod game;

struct AppState {
    game: Mutex<GameState>,
}

#[derive(Deserialize)]
struct MoveRequest {
    player_id: u32,
    target_x: u8,
    target_y: u8,
}

async fn roll_dice_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut game = state.game.lock().unwrap();
    let _ = game.roll_dice();

    Json(game.clone())
}

async fn move_player_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MoveRequest>,
) -> Response {
    let mut game = state.game.lock().unwrap();

    match game.try_move(payload.player_id, payload.target_x, payload.target_y) {
        Ok(_) => (StatusCode::OK, Json(game.clone())).into_response(),
        Err(err_msg) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": err_msg })),
        )
            .into_response(),
    }
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
