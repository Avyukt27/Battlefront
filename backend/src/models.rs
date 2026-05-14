use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub colour: PlayerColour,
    pub x: u8,
    pub y: u8,
    pub health: i32,
    pub max_health: i32,
    pub shield: i32,
    pub status_effects: Vec<ActiveEffect>,
    pub class: PlayerClass,
    pub cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Status {
    Bleed,
    Poison,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveEffect {
    pub status: Status,
    pub duration: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerColour {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerClass {
    Gunslinger,
    Arsenist,
    Mage,
    Knight,
    Assassin,
}

impl PlayerClass {
    pub fn get_signature_card(&self) -> Card {
        match self {
            PlayerClass::Gunslinger => Card::create_revolver(),
            _ => Card::create_stick(),
        }
    }
}
