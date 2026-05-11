use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use backend::{AppState, game::GameState, routes::create_routes};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        game: Mutex::new(GameState::new(8, 8)),
    });

    let app = create_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
