pub mod game;
pub mod requests;
pub mod routes;

use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct AppState {
    pub game: Mutex<game::GameState>,
}

pub struct LobbyManager {
    pub games: Mutex<HashMap<String, Arc<AppState>>>
}
