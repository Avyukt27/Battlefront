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
    requests::{MoveRequest, PlayerRequest, UseCardRequest},
};

pub fn create_routes(state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/api/state/{game_id}", get(get_state_handler))
        .route("/api/roll/{game_id}", post(roll_dice_handler))
        .route("/api/move/{game_id}", post(move_player_handler))
        .route("/api/join/{game_id}", post(join_game_handler))
        .route("/api/leave/{game_id}", post(leave_game_handler))
        .route("/api/create", post(create_game_handler))
        .route("/api/draw/{game_id}", post(draw_card_handler))
        .route("/api/use/{game_id}", post(use_card_handler))
        .route("/api/end_turn/{game_id}", post(end_turn_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn roll_dice_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    let _ = game.roll_dice();
    Ok(Json(game.clone()))
}

pub async fn move_player_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<MoveRequest>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    match game.try_move(payload.player_id, payload.target_x, payload.target_y) {
        Ok(_) => Ok(Json(game.clone())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub async fn get_state_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    Ok(Json(game.clone()))
}

pub async fn join_game_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    match game.add_player() {
        Ok(new_player) => Ok(Json(json![{"player_id": new_player.id, "state": *game}])),
        Err(e) => Err((StatusCode::NOT_ACCEPTABLE, e)),
    }
}

pub async fn leave_game_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<PlayerRequest>,
) -> Result<Json<Option<GameState>>, (StatusCode, String)> {
    let mut games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    let player_colour = game
        .players
        .iter()
        .find(|p| p.id == payload.player_id)
        .map(|p| p.colour.clone());

    if let Some(colour) = player_colour
        && colour == game.current_turn
    {
        game.next_turn();
    }

    game.players.retain(|p| p.id != payload.player_id);

    if game.players.is_empty() {
        drop(game);
        games.remove(&game_id);
        return Ok(Json(None));
    }

    Ok(Json(Some(game.clone())))
}

pub async fn create_game_handler(State(state): State<Arc<ServerState>>) -> Json<Value> {
    let mut games = state.games.lock().unwrap();
    let game_id = format!("{:x}", rand::random::<u16>());
    let mut new_game = GameState::new(8, 8);
    new_game.initialise_deck();
    games.insert(game_id.clone(), Arc::new(Mutex::new(new_game)));
    Json(json!({ "game_id": game_id }))
}

pub async fn draw_card_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<PlayerRequest>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    let can_draw = game
        .players
        .iter()
        .find(|p| p.id == payload.player_id)
        .map(|p| p.cards.len() < 3)
        .unwrap_or(false);

    if !can_draw {
        return Err((StatusCode::BAD_REQUEST, "Cannot draw cards!".to_string()));
    }

    if let Some(mut new_card) = game.deck.pop() {
        new_card.id = uuid::Uuid::new_v4().to_string();
        if let Some(player) = game.players.iter_mut().find(|p| p.id == payload.player_id) {
            player.cards.push(new_card);
        }
    }

    Ok(Json(game.clone()))
}

pub async fn use_card_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<UseCardRequest>,
) -> Result<Json<(GameState, bool)>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();

    let attacker_colour = game
        .players
        .iter()
        .find(|p| p.id == payload.attacker_id)
        .ok_or((StatusCode::NOT_FOUND, "Player not found".to_string()))?
        .colour
        .clone();

    if game.current_turn != attacker_colour {
        return Err((StatusCode::FORBIDDEN, "Not your turn!".to_string()));
    }

    match game.use_card(&payload.card_id, payload.attacker_id, payload.target_pos) {
        Ok(h) => Ok(Json((game.clone(), h))),
        Err(e) => Err((StatusCode::FORBIDDEN, e)),
    }
}

pub async fn end_turn_handler(
    Path(game_id): Path<String>,
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<PlayerRequest>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let games = state.games.lock().unwrap();
    let mut game = games
        .get(&game_id)
        .ok_or((StatusCode::NOT_FOUND, "Game not found".to_string()))?
        .lock()
        .unwrap();
    let player_colour = game
        .players
        .iter()
        .find(|p| p.id == payload.player_id)
        .ok_or((StatusCode::NOT_FOUND, "Player not found".to_string()))?
        .colour
        .clone();

    if player_colour != game.current_turn {
        return Err((StatusCode::FORBIDDEN, "Not your turn!".to_string()));
    }

    game.next_turn();
    Ok(Json(game.clone()))
}
