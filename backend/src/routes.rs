use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

use crate::{
    AppState,
    requests::{JoinRequest, MoveRequest},
};

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/state", get(get_state_handler))
        .route("/api/roll", post(roll_dice_handler))
        .route("/api/move", post(move_player_handler))
        .route("/api/join", post(add_player_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
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

pub async fn get_state_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let game = state.game.lock().unwrap();
    Json(game.clone())
}

pub async fn add_player_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JoinRequest>,
) -> impl IntoResponse {
    let mut game = state.game.lock().unwrap();

    let new_id = (game.players.len() as u32) + 1;

    game.add_player(new_id, payload.colour, payload.class);

    (StatusCode::CREATED, Json(game.clone()))
}
