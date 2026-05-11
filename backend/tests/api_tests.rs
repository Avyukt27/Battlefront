use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

use backend::AppState;
use backend::game::GameState;
use backend::routes::create_routes;

#[tokio::test]
async fn test_join() {
    let state = Arc::new(AppState {
        game: Mutex::new(GameState::new(8, 8)),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/join")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "name": "Test Player",
                "class": "Knight",
                "colour": "Red"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["players"].as_array().unwrap().len(), 1);
    assert_eq!(body["players"][0]["colour"], "Red");
}

#[tokio::test]
async fn test_roll() {
    let state = Arc::new(AppState {
        game: Mutex::new(GameState::new(8, 8)),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/roll")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let roll = body["last_roll"]
        .as_u64()
        .expect("last_roll should be a number");
    assert!(roll >= 1 && roll <= 6, "Roll was {}, expected 1-6", roll);
}

#[tokio::test]
async fn test_cannot_move_before_rolling() {
    let state = Arc::new(AppState {
        game: Mutex::new(GameState::new(8, 8)),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/move")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({"player_id": 1, "target_x": 1, "target_y": 1}).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_cannot_move_occupied() {
    let mut game = GameState::new(8, 8);
    game.add_player(1, backend::game::PlayerColour::Red, "Artificer".to_string());
    game.add_player(2, backend::game::PlayerColour::Blue, "Knight".to_string());
    game.last_roll = 14;
    game.current_turn = backend::game::PlayerColour::Red;

    let state = Arc::new(AppState {
        game: Mutex::new(game),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/move")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({"player_id": 1, "target_x": 7, "target_y": 7}).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_cannot_move_out_of_turn() {
    let mut game = GameState::new(8, 8);
    game.add_player(1, backend::game::PlayerColour::Red, "Artificer".to_string());
    game.add_player(2, backend::game::PlayerColour::Blue, "Knight".to_string());
    game.last_roll = 14;
    game.current_turn = backend::game::PlayerColour::Blue;

    let state = Arc::new(AppState {
        game: Mutex::new(game),
    });
    let app = create_routes(state);

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/move")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({"player_id": 1, "target_x": 2, "target_y": 2}).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
