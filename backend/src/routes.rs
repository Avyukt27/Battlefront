use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct MoveRequest {
    player_id: u32,
    target_x: u8,
    target_y: u8,
}

pub async fn roll_dice_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut game = state.game.lock().unwrap();
    let _ = game.roll_dice();

    Json(game.clone())
}

pub async fn move_player_handler(
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
