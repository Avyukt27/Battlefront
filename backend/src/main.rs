use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use backend::{ServerState, game::GameState, routes::create_routes};

#[tokio::main]
async fn main() {
    let mut games = HashMap::new();
    games.insert(
        "test-game".to_string(),
        Arc::new(Mutex::new(GameState::new(8, 8))),
    );

    let state = Arc::new(ServerState {
        games: Mutex::new(games),
    });

    let app = create_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
