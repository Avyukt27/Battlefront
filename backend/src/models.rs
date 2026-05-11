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

impl Card {
    pub fn create_rock() -> Self {
        Self {
            name: "Rock".to_string(),
            effects: vec![
                CardEffect::SkillCheck {
                    threshold: 5,
                    max_range: 4,
                },
                CardEffect::Damage { power: 1 },
            ],
        }
    }

    pub fn create_stick() -> Self {
        Self {
            name: "Sword".to_string(),
            effects: vec![CardEffect::Damage { power: 2 }],
        }
    }

    pub fn create_bandage() -> Self {
        Self {
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
