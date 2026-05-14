use serde::{Deserialize, Serialize};

use crate::models::{CardEffect, Status};

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
            effects: vec![
                CardEffect::Damage { power: 1 },
                CardEffect::Range { max_range: 1 },
            ],
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
                CardEffect::Range { max_range: 0 },
            ],
        }
    }

    pub fn create_shield() -> Self {
        Self {
            id: "".to_string(),
            name: "Shield".to_string(),
            effects: vec![
                CardEffect::Shield { value: 1 },
                CardEffect::Range { max_range: 0 },
            ],
        }
    }
}
