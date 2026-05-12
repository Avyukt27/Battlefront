use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::models::{ActiveEffect, Card, CardEffect, Status};

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
    pub last_roll: i16,
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

    pub fn roll_dice(&mut self) -> i16 {
        let modifier = if self.players.iter().any(|p| {
            p.colour == self.current_turn
                && p.status_effects.iter().any(|e| e.status == Status::Bleed)
        }) {
            -1i16
        } else {
            0i16
        };

        self.last_roll = rand::random_range(1..=6);
        (self.last_roll + modifier).max(0)
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

        Ok(())
    }

    pub fn next_turn(&mut self) {
        if let Some(player) = self
            .players
            .iter_mut()
            .find(|p| p.colour == self.current_turn)
        {
            for effect in player.status_effects.iter_mut() {
                match effect.status {
                    Status::Poison => player.health = player.health.saturating_sub(1),
                    _ => {}
                }
                effect.duration = effect.duration.saturating_sub(1);
            }

            player.status_effects.retain(|e| e.duration > 0);
        }

        if let Some(index) = self
            .players
            .iter()
            .position(|p| p.colour == self.current_turn)
        {
            let next_index = (index + 1) % self.players.len();
            self.current_turn = self.players[next_index].colour.clone();
        }
    }

    pub fn add_player(&mut self, id: u32, colour: PlayerColour, class: String) {
        let (start_x, start_y) = match colour {
            PlayerColour::Red => (0, 0),
            PlayerColour::Blue => (self.width - 1, self.height - 1),
            PlayerColour::Green => (0, self.height - 1),
        };

        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..3 {
            if let Some(mut card) = self.deck.pop() {
                card.id = uuid::Uuid::new_v4().to_string();
                cards.push(card);
            }
        }

        self.players.push(Player {
            id,
            colour,
            x: start_x,
            y: start_y,
            health: 20,
            max_health: 20,
            status_effects: Vec::new(),
            class,
            cards,
        });
    }

    pub fn use_card(&mut self, card_id: &str, attacker_id: u32, target_pos: (u8, u8)) {
        let mut card_to_use: Option<Card> = None;
        let mut attacker_pos = (0u8, 0u8);

        if let Some(attacker) = self.players.iter_mut().find(|p| p.id == attacker_id) {
            attacker_pos = (attacker.x, attacker.y);
            if let Some(idx) = attacker.cards.iter().position(|c| c.id == card_id) {
                card_to_use = Some(attacker.cards.remove(idx));
            }
        }

        let card = match card_to_use {
            Some(c) => c,
            None => return,
        };

        let distance = (attacker_pos.0 as i16 - target_pos.0 as i16).abs()
            + (attacker_pos.1 as i16 - target_pos.1 as i16).abs();

        let mut hit_landed = true;
        for effect in &card.effects {
            if let CardEffect::SkillCheck {
                threshold,
                max_range,
            } = effect
            {
                if distance > *max_range as i16 {
                    hit_landed = false;
                } else if distance > 1 {
                    let roll = rand::random_range(1..=6) as u8;
                    if roll < *threshold {
                        hit_landed = false;
                    }
                }
            }
        }

        if !hit_landed {
            return;
        }

        let radius = if card.name == "Poison Bomb" {
            1i16
        } else {
            0i16
        };

        for player in self.players.iter_mut() {
            let dx = (player.x as i16 - target_pos.0 as i16).abs();
            let dy = (player.y as i16 - target_pos.1 as i16).abs();
            let dist_to_impact = dx.max(dy);

            if dist_to_impact <= radius {
                for effect in &card.effects {
                    match effect {
                        CardEffect::Damage { power } => {
                            player.health = player.health.saturating_sub(*power);
                        }
                        CardEffect::ApplyStatus { status, duration } => {
                            if let Some(s) = player
                                .status_effects
                                .iter_mut()
                                .find(|e| e.status == *status)
                            {
                                s.duration = s.duration.max(*duration);
                            } else {
                                player.status_effects.push(ActiveEffect {
                                    status: *status,
                                    duration: *duration,
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }

            if player.id == attacker_id {
                for effect in &card.effects {
                    match effect {
                        CardEffect::Heal { amount } => {
                            player.health = (player.health + amount).min(player.max_health);
                        }
                        CardEffect::CureStatus { status } => {
                            player.status_effects.retain(|e| e.status != *status);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn initialise_deck(&mut self) {
        let mut new_deck: Vec<Card> = Vec::new();

        for _ in 0..4 {
            new_deck.push(Card::create_rock());
        }
        for _ in 0..5 {
            new_deck.push(Card::create_stick());
        }
        for _ in 0..3 {
            new_deck.push(Card::create_bandage());
        }

        let mut rng = rand::rng();
        new_deck.shuffle(&mut rng);

        self.deck = new_deck;
    }
}
