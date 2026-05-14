use rand::seq::SliceRandom;
use serde::Serialize;

use crate::{
    card::Card,
    models::{ActiveEffect, CardEffect, Player, PlayerColour, Status},
};

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

    pub fn add_player(&mut self) -> Result<Player, String> {
        let all_colours = vec![
            PlayerColour::Red,
            PlayerColour::Blue,
            PlayerColour::Green,
            PlayerColour::Yellow,
        ];

        let colour = all_colours
            .into_iter()
            .find(|c| !self.players.iter().any(|p| p.colour == *c))
            .ok_or("Lobby is full".to_string())?;

        let (start_x, start_y) = match colour {
            PlayerColour::Red => (0, 0),
            PlayerColour::Blue => (self.width - 1, self.height - 1),
            PlayerColour::Green => (0, self.height - 1),
            PlayerColour::Yellow => (self.width - 1, 0),
        };

        let new_id = self.players.iter().map(|p| p.id).max().unwrap_or(0) + 1;

        let classes = vec!["Gunslinger", "Mage", "Knight", "Assassin", "Arsenist"];
        let taken_classes: Vec<String> = self.players.iter().map(|p| p.class.clone()).collect();
        let available_classes: Vec<&&str> = classes
            .iter()
            .filter(|class| !taken_classes.contains(&class.to_string()))
            .collect();

        if available_classes.is_empty() {
            return Err("No classes available".to_string());
        }

        let class_index = rand::random_range(0..available_classes.len());
        let class = available_classes[class_index].to_string();

        let mut cards = Vec::new();
        for _ in 0..3 {
            if let Some(mut card) = self.deck.pop() {
                card.id = uuid::Uuid::new_v4().to_string();
                cards.push(card);
            }
        }

        let player = Player {
            id: new_id,
            colour,
            x: start_x,
            y: start_y,
            health: 20,
            max_health: 20,
            shield: 0,
            status_effects: Vec::new(),
            class,
            cards,
        };

        self.players.push(player.clone());
        Ok(player)
    }

    pub fn use_card(
        &mut self,
        card_id: &str,
        attacker_id: u32,
        target_pos: (u8, u8),
    ) -> Result<bool, String> {
        let (attacker_pos, card_effects, card_name) = {
            let attacker = self
                .players
                .iter()
                .find(|p| p.id == attacker_id)
                .ok_or("Player not found")?;

            let card = attacker
                .cards
                .iter()
                .find(|c| c.id == card_id)
                .ok_or("Card not found in hand")?;

            (
                (attacker.x, attacker.y),
                card.effects.clone(),
                card.name.clone(),
            )
        };

        let distance = (attacker_pos.0 as i16 - target_pos.0 as i16).abs()
            + (attacker_pos.1 as i16 - target_pos.1 as i16).abs();

        for effect in &card_effects {
            if let CardEffect::Range { max_range } = effect {
                if distance > *max_range as i16 {
                    return Err("Out of range!".to_string());
                }
            }
        }
        if let Some(attacker) = self.players.iter_mut().find(|p| p.id == attacker_id) {
            if let Some(idx) = attacker.cards.iter().position(|c| c.id == card_id) {
                attacker.cards.remove(idx);
            }
        }

        let mut hit_landed = true;
        for effect in &card_effects {
            if let CardEffect::SkillCheck { threshold } = effect {
                if distance > 1 {
                    let roll = rand::random_range(1..=6) as u8;
                    if roll < *threshold {
                        hit_landed = false;
                    }
                }
            }
        }
        if !hit_landed {
            return Ok(false);
        }

        let radius = if card_name == "Poison Bomb" {
            1i16
        } else {
            0i16
        };

        let mut damage_mod = 0;
        for player in self.players.iter_mut() {
            let dx = (player.x as i16 - target_pos.0 as i16).abs();
            let dy = (player.y as i16 - target_pos.1 as i16).abs();
            let dist_to_impact = dx.max(dy);

            if dist_to_impact <= radius {
                for effect in &card_effects {
                    match effect {
                        CardEffect::Ability { name } => match name.as_str() {
                            _ => {}
                        },
                        CardEffect::Damage { power } => {
                            if player.shield >= *power + damage_mod {
                                player.shield -= *power + damage_mod;
                            } else {
                                let overflow = *power + damage_mod - player.shield;
                                player.shield = 0;
                                player.health = player.health.saturating_sub(overflow);
                            }
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
                for effect in &card_effects {
                    match effect {
                        CardEffect::Heal { amount } => {
                            player.health = (player.health + amount).min(player.max_health);
                        }
                        CardEffect::CureStatus { status } => {
                            player.status_effects.retain(|e| e.status != *status);
                        }
                        CardEffect::Shield { value } => {
                            player.shield = (player.shield + *value).min(5);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(true)
    }

    pub fn initialise_deck(&mut self) {
        let mut new_deck: Vec<Card> = Vec::new();

        for _ in 0..4 {
            new_deck.push(Card::create_stone());
        }
        for _ in 0..5 {
            new_deck.push(Card::create_stick());
        }
        for _ in 0..3 {
            new_deck.push(Card::create_bandage());
        }
        for _ in 0..4 {
            new_deck.push(Card::create_shield());
        }

        let mut rng = rand::rng();
        new_deck.shuffle(&mut rng);

        self.deck = new_deck;
    }
}
