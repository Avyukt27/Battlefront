use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Bleed,
    Poison,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CardEffect {
    Damage { power: i32 },
    Heal { amount: i32 },
    SkillCheck { threshold: u8, max_range: u8 },
    ApplyStatus { status: Status, duration: u8 },
    CureStatus { status: Status },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub name: String,
    pub effects: Vec<CardEffect>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveEffect {
    pub status: Status,
    pub duration: u8,
}
