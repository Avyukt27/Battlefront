use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerColour {
    Red,
    Blue,
    Green,
    Yellow,
}

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
    pub class: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Status {
    Bleed,
    Poison,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CardEffect {
    Damage { power: i32 },
    Heal { amount: i32 },
    SkillCheck { threshold: u8 },
    ApplyStatus { status: Status, duration: u8 },
    CureStatus { status: Status },
    Range { max_range: u8 },
    Shield { value: i32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveEffect {
    pub status: Status,
    pub duration: u8,
}
