use serde::{Deserialize, Serialize};

use crate::models::{ActiveEffect, Card, CardEffect};

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
    pub x: u8,
    pub y: u8,
    pub health: i32,
    pub max_health: i32,
    pub status_effects: Vec<ActiveEffect>,
    pub class: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: PlayerColour,
    pub last_roll: u8,
    pub width: u8,
    pub height: u8,
    pub deck: Vec<Card>,
}

impl GameState {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            players: Vec::new(),
            current_turn: PlayerColour::Red,
            last_roll: 0,
            width,
            height,
            deck: Vec::new(),
        }
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == id)
    }

    pub fn roll_dice(&mut self) -> u8 {
        self.last_roll = rand::random_range(1..=6);
        self.last_roll
    }

    pub fn try_move(&mut self, player_id: u32, target_x: u8, target_y: u8) -> Result<(), String> {
        let (current_x, current_y, player_colour) = {
            let p = self
                .players
                .iter()
                .find(|p| p.id == player_id)
                .ok_or("Player not found")?;
            (p.x, p.y, p.colour.clone())
        };

        if player_colour != self.current_turn {
            return Err("It is not your turn!".to_string());
        }
        if self.last_roll == 0 {
            return Err("You must roll the dice first!".to_string());
        }

        let dist =
            (current_x as i16 - target_x as i16).abs() + (current_y as i16 - target_y as i16).abs();

        if dist > self.last_roll as i16 {
            return Err("Destination is too far away!".to_string());
        }
        if self
            .players
            .iter()
            .any(|p| p.x == target_x && p.y == target_y)
        {
            return Err("Tile is already occupied!".to_string());
        }

        let player = self.get_player_mut(player_id).unwrap();
        player.x = target_x;
        player.y = target_y;

        self.last_roll = 0;
        if let Some(index) = self
            .players
            .iter()
            .position(|p| p.colour == self.current_turn)
        {
            let next_index = (index + 1) % self.players.len();
            self.current_turn = self.players[next_index].colour.clone();
        }

        Ok(())
    }

    pub fn add_player(&mut self, id: u32, colour: PlayerColour, class: String) {
        let (start_x, start_y) = match colour {
            PlayerColour::Red => (0, 0),
            PlayerColour::Blue => (self.width - 1, self.height - 1),
            PlayerColour::Green => (0, self.height - 1),
        };

        self.players.push(Player {
            id,
            colour,
            x: start_x,
            y: start_y,
            health: 20,
            max_health: 20,
            status_effects: Vec::new(),
            class,
            cards: Vec::new(),
        });
    }

    pub fn use_card(card: &Card, attacker: &mut Player, target: &mut Player) {
        let mut hit_confirmed = true;
        let distance = (attacker.x as i16 - target.x as i16).abs()
            + (attacker.y as i16 - target.y as i16).abs();

        for effect in &card.effects {
            if !hit_confirmed {
                break;
            }

            match effect {
                CardEffect::SkillCheck {
                    threshold,
                    max_range,
                } => {
                    if distance > *max_range as i16 {
                        hit_confirmed = false;
                    } else if distance > 1 {
                        let roll = rand::random_range(1..=6) as u8;
                        if roll < *threshold {
                            hit_confirmed = false;
                        }
                    }
                }
                CardEffect::Damage { power } => target.health -= power,
                CardEffect::Heal { amount } => {
                    attacker.health = (attacker.health + amount).min(attacker.max_health)
                }
                CardEffect::ApplyStatus { status, duration } => {
                    target.status_effects.push(ActiveEffect {
                        status: status.clone(),
                        duration: *duration,
                    })
                }
                CardEffect::CureStatus { status } => {
                    attacker.status_effects.retain(|e| e.status != *status);
                }
            }
        }
    }

    pub fn initialise_deck(&mut self) {
        let mut new_deck = Vec::new();

        for _ in 0..4 {}
    }
}
