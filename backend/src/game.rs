use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerColour {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub colour: PlayerColour,
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub occupant: Option<Player>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GameState {
    pub board: Vec<Tile>,
    pub current_turn: PlayerColour,
    pub width: u8,
    pub height: u8,
}

impl GameState {
    pub fn new(width: u8, height: u8) -> Self {
        let mut board = Vec::new();
        for y in 0..height {
            for x in 0..width {
                board.push(Tile {
                    x,
                    y,
                    occupant: None,
                });
            }
        }

        Self {
            board,
            current_turn: PlayerColour::Red,
            width,
            height,
        }
    }

    pub fn get_tile_mut(&mut self, x: u8, y: u8) -> Option<&mut Tile> {
        self.board.iter_mut().find(|t| t.x == x && t.y == y)
    }
}
