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
