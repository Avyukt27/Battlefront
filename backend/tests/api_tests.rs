use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower::ServiceExt;

use backend::ServerState;
use backend::game::GameState;
use backend::routes::create_routes;

const TEST_ID: &str = "test-game";

fn setup_test_state(game: GameState) -> Arc<ServerState> {
    let mut games = HashMap::new();
    games.insert(TEST_ID.to_string(), Arc::new(Mutex::new(game)));
    Arc::new(ServerState {
        games: Mutex::new(games),
    })
}

#[tokio::test]
async fn test_join() {
    let state = setup_test_state(GameState::new(8, 8));
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri(format!("/api/join/{}", TEST_ID))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "class": "Knight"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["state"]["players"].as_array().unwrap().len(), 1);
    assert_eq!(body["state"]["players"][0]["colour"], "Red");
}

#[tokio::test]
async fn test_roll() {
    let state = setup_test_state(GameState::new(8, 8));
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri(format!("/api/roll/{}", TEST_ID))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let roll = body["lastRoll"]
        .as_u64()
        .expect("last_roll should be a number");
    assert!(roll >= 1 && roll <= 6);
}

#[tokio::test]
async fn test_cannot_move_before_rolling() {
    let state = setup_test_state(GameState::new(8, 8));
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri(format!("/api/move/{}", TEST_ID))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({"playerId": 1, "targetX": 1, "targetY": 1}).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_automatic_turn_change() {
    let mut game = GameState::new(8, 8);
    let _ = game.add_player();
    let _ = game.add_player();
    game.last_roll = 2;
    game.current_turn = backend::models::PlayerColour::Red;

    let state = setup_test_state(game);
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri(format!("/api/end/{}", TEST_ID))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(json!({"playerId": 1}).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["currentTurn"], "Blue");
}

#[tokio::test]
async fn test_create_game() {
    let state = Arc::new(ServerState {
        games: Mutex::new(HashMap::new()),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/create")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
