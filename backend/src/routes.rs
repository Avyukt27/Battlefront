use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};
use tower_http::cors::CorsLayer;

use crate::{
    ServerState,
    game::GameState,
    requests::{JoinRequest, MoveRequest},
};

pub fn create_routes(state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/api/state", get(get_state_handler))
        .route("/api/roll", post(roll_dice_handler))
        .route("/api/move", post(move_player_handler))
        .route("/api/join", post(add_player_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn roll_dice_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<GameState>, StatusCode> {
    let games = state.games.lock().unwrap();

    if let Some(game_mutex) = games.get(&game_id) {
        let mut game = game_mutex.lock().unwrap();
        let _ = game.roll_dice();
        Ok(Json(game.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn move_player_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<MoveRequest>,
) -> Result<Json<GameState>, StatusCode> {
    let games = state.games.lock().unwrap();

    if let Some(game_mutex) = games.get(&game_id) {
        let mut game = game_mutex.lock().unwrap();

        match game.try_move(payload.player_id, payload.target_x, payload.target_y) {
            Ok(_) => Ok(Json(game.clone())),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_state_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<GameState>, StatusCode> {
    let games = state.games.lock().unwrap();

    if let Some(game_mutex) = games.get(&game_id) {
        let game = game_mutex.lock().unwrap();
        Ok(Json(game.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn add_player_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<JoinRequest>,
) -> Result<Json<GameState>, StatusCode> {
    let games = state.games.lock().unwrap();

    if let Some(game_mutex) = games.get(&game_id) {
        let mut game = game_mutex.lock().unwrap();
        let new_id = (game.players.len() as u32) + 1;
        game.add_player(new_id, payload.colour, payload.class);
        Ok(Json(game.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_game(State(state): State<Arc<ServerState>>) -> Json<Value> {
    let mut games = state.games.lock().unwrap();
    let game_id = format!("{:x}", rand::random::<u16>());
    let new_game = Arc::new(Mutex::new(GameState::new(8, 8)));
    games.insert(game_id.clone(), new_game);
    Json(json!({ "game_id": game_id }))
}
