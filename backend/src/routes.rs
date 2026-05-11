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
    game::{GameState, PlayerColour},
    models::Card,
    requests::{JoinRequest, MoveRequest},
};

pub fn create_routes(state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/api/state/{game_id}", get(get_state_handler))
        .route("/api/roll/{game_id}", post(roll_dice_handler))
        .route("/api/move/{game_id}", post(move_player_handler))
        .route("/api/join/{game_id}", post(join_game_handler))
        .route("/api/create", post(create_game_handler))
        .route("/api/cards/{game_id}/{player_id}", get(get_cards_handler))
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

pub async fn join_game_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<JoinRequest>,
) -> Result<Json<Value>, StatusCode> {
    let games = state.games.lock().unwrap();

    if let Some(game_mutex) = games.get(&game_id) {
        let mut game = game_mutex.lock().unwrap();
        let new_id = (game.players.len() as u32) + 1;

        let player_count = game.players.len();
        let player_colour = match player_count {
            0 => Some(PlayerColour::Red),
            1 => Some(PlayerColour::Blue),
            2 => Some(PlayerColour::Green),
            _ => None,
        };
        if let Some(colour) = player_colour {
            game.add_player(new_id, colour, payload.class);
            Ok(Json(json!({"player_id": new_id, "state": *game})))
        } else {
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_game_handler(State(state): State<Arc<ServerState>>) -> Json<Value> {
    let mut games = state.games.lock().unwrap();
    let game_id = format!("{:x}", rand::random::<u16>());
    let new_game = Arc::new(Mutex::new(GameState::new(8, 8)));
    games.insert(game_id.clone(), new_game);
    Json(json!({ "game_id": game_id }))
}

pub async fn get_cards_handler(
    Path((game_id, player_id)): Path<(String, u32)>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<Vec<Card>>, StatusCode> {
    let games = state.games.lock().unwrap();
    let game = games
        .get(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .lock()
        .unwrap();

    let player = game
        .players
        .iter()
        .find(|p| p.id == player_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(player.cards.clone()))
}
