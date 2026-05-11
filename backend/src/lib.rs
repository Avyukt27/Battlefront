pub mod game;
pub mod requests;
pub mod routes;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::game::GameState;

pub struct ServerState {
    pub games: Mutex<HashMap<String, Arc<Mutex<GameState>>>>,
}
