use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "PascalCase")]
pub enum Status {
    Fracture,
    Poison,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveEffect {
    pub status: Status,
    pub duration: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PlayerColour {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
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
            Self::Gunslinger => Card::create_revolver(),
            Self::Mage => Card::create_staff(),
            Self::Assassin => Card::create_dagger(),
            _ => Card::create_stick(),
        }
    }
}
