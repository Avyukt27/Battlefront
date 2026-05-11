pub mod game;
pub mod requests;
pub mod routes;

use std::sync::Mutex;

pub struct AppState {
    pub game: Mutex<game::GameState>,
}
