use serde::{Deserialize, Serialize};

use crate::models::Status;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub name: String,
    pub is_signature: bool,
    pub cooldown: u8,
    pub effects: Vec<CardEffect>,
}

impl Card {
    pub fn create_stone() -> Self {
        Self {
            id: "".to_string(),
            name: "Stone".to_string(),
            is_signature: false,
            cooldown: 0,
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
            is_signature: false,
            cooldown: 0,
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
            is_signature: false,
            cooldown: 0,
            effects: vec![
                CardEffect::Heal { amount: 2 },
                CardEffect::CureStatus {
                    status: Status::Fracture,
                },
                CardEffect::Range { max_range: 0 },
            ],
        }
    }

    pub fn create_shield() -> Self {
        Self {
            id: "".to_string(),
            name: "Shield".to_string(),
            is_signature: false,
            cooldown: 0,
            effects: vec![
                CardEffect::Shield { value: 1 },
                CardEffect::Range { max_range: 0 },
            ],
        }
    }

    pub fn create_revolver() -> Self {
        Self {
            id: "".to_string(),
            name: "Revolver".to_string(),
            is_signature: true,
            cooldown: 0,
            effects: vec![
                CardEffect::Damage { power: 2 },
                CardEffect::Range { max_range: 5 },
                CardEffect::Ability {
                    ability: CardAbility::DamageMul {
                        multiplier: 3.0,
                        threshold: 0,
                    },
                    cooldown: 3,
                },
                CardEffect::SkillCheck { threshold: 4 },
            ],
        }
    }

    pub fn create_staff() -> Self {
        Self {
            id: "".to_string(),
            name: "Staff".to_string(),
            is_signature: true,
            cooldown: 0,
            effects: vec![
                CardEffect::Damage { power: 2 },
                CardEffect::Range { max_range: 5 },
                CardEffect::Ability {
                    ability: CardAbility::DamageMul {
                        multiplier: 2.0,
                        threshold: 4,
                    },
                    cooldown: 2,
                },
            ],
        }
    }

    pub fn create_dagger() -> Self {
        Self {
            id: "".to_string(),
            name: "Dagger".to_string(),
            is_signature: true,
            cooldown: 0,
            effects: vec![
                CardEffect::Damage { power: 2 },
                CardEffect::Range { max_range: 1 },
                CardEffect::Ability {
                    ability: CardAbility::ShieldPierce,
                    cooldown: 2,
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CardEffect {
    Damage { power: i32 },
    Heal { amount: i32 },
    SkillCheck { threshold: u8 },
    ApplyStatus { status: Status, duration: u8 },
    CureStatus { status: Status },
    Range { max_range: u8 },
    Shield { value: i32 },
    Ability { ability: CardAbility, cooldown: u8 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CardAbility {
    DamageMul { multiplier: f32, threshold: u8 },
    ShieldPierce,
}
