use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub effects: Vec<CardEffect>,
}

impl Card {
    pub fn create_stone() -> Self {
        Self {
            id: "".to_string(),
            name: "Stone".to_string(),
            effects: vec![
                CardEffect::SkillCheck { threshold: 5 },
                CardEffect::Damage { power: 1 },
                CardEffect::Range { max_range: 4 },
            ],
        }
    }

    pub fn create_stick() -> Self {
        Self {
            id: "".to_string(),
            name: "Stick".to_string(),
            effects: vec![CardEffect::Damage { power: 1 }],
        }
    }

    pub fn create_bandage() -> Self {
        Self {
            id: "".to_string(),
            name: "Bandage".to_string(),
            effects: vec![
                CardEffect::Heal { amount: 2 },
                CardEffect::CureStatus {
                    status: Status::Bleed,
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveEffect {
    pub status: Status,
    pub duration: u8,
}
