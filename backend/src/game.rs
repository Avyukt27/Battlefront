use rand::seq::SliceRandom;
use serde::Serialize;

use crate::{
    card::{Card, CardAbility, CardEffect},
    models::{ActiveEffect, FireTile, Player, PlayerClass, PlayerColour, Status},
};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: PlayerColour,
    pub last_roll: i16,
    pub width: u8,
    pub height: u8,
    pub fire_tiles: Vec<FireTile>,
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
            fire_tiles: Vec::new(),
            deck: Vec::new(),
        }
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == id)
    }

    pub fn roll_dice(&mut self) -> i16 {
        let modifier = if self.players.iter().any(|p| {
            p.colour == self.current_turn
                && p.status_effects
                    .iter()
                    .any(|e| e.status == Status::Fracture)
        }) {
            -1i16
        } else {
            0i16
        };

        self.last_roll = rand::random_range(1..=6);
        (self.last_roll + modifier).max(1)
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

        if dist > self.last_roll {
            return Err("Destination is too far!".to_string());
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

            for card in player.cards.iter_mut() {
                card.cooldown = card.cooldown.saturating_sub(1);
            }
        }

        self.fire_tiles.retain_mut(|tile| {
            if tile.duration > 0 {
                tile.duration -= 1;
                true
            } else {
                false
            }
        });
        for player in &mut self.players {
            if self
                .fire_tiles
                .iter()
                .any(|f| f.x == player.x && f.y == player.y)
            {
                player.health = player.health.saturating_sub(1);
            }
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

        let classes = vec![
            PlayerClass::Gunslinger,
            PlayerClass::Mage,
            PlayerClass::Knight,
            PlayerClass::Assassin,
            PlayerClass::Arsenist,
        ];
        let taken_classes: Vec<PlayerClass> =
            self.players.iter().map(|p| p.class.clone()).collect();
        let available_classes: Vec<&PlayerClass> = classes
            .iter()
            .filter(|class| !taken_classes.contains(&class))
            .collect();

        if available_classes.is_empty() {
            return Err("No classes available".to_string());
        }

        let class_index = rand::random_range(0..available_classes.len());
        let given_class = available_classes[class_index];

        let mut cards: Vec<Card> = Vec::new();
        let mut sig_cards = given_class.get_signature_cards();
        for sig_card in sig_cards.iter_mut() {
            sig_card.id = uuid::Uuid::new_v4().to_string();
            cards.push(sig_card.clone());
        }

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
            class: given_class.clone(),
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
        use_ability: bool,
    ) -> Result<bool, String> {
        let (attacker_pos, card_cooldown, card_effects, card_name) = {
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
                card.cooldown,
                card.effects.clone(),
                card.name.clone(),
            )
        };

        let mut damage_mod = 1.0;
        let mut shield_bypass = false;
        let mut life_stolen = 0;
        let has_lifesteal = card_effects
            .iter()
            .any(|e| matches!(e, CardEffect::LifeSteal));

        if card_cooldown > 0 {
            return Err("Card is on cooldown".to_string());
        }

        if use_ability {
            for effect in &card_effects {
                if let CardEffect::Ability { ability, .. } = effect {
                    match ability {
                        CardAbility::DamageMul {
                            multiplier,
                            threshold,
                        } => {
                            let roll = rand::random_range(1..=6) as u8;
                            damage_mod = if roll >= *threshold {
                                *multiplier
                            } else {
                                1.0 / *multiplier
                            };
                        }
                        CardAbility::ShieldPierce => {
                            shield_bypass = true;
                        }
                    }
                }
            }
        }

        let distance = (attacker_pos.0 as i16 - target_pos.0 as i16).abs()
            + (attacker_pos.1 as i16 - target_pos.1 as i16).abs();

        for effect in &card_effects {
            if let CardEffect::Range { max_range } = effect {
                if distance > *max_range as i16 {
                    return Err("Out of range!".to_string());
                }
            }
            if let CardEffect::Ignite = effect {
                let patterns = vec![
                    (target_pos.0, target_pos.1),
                    (target_pos.0, target_pos.1.saturating_add(1)),
                    (target_pos.0, target_pos.1.saturating_sub(1)),
                    (target_pos.0.saturating_add(1), target_pos.1),
                    (target_pos.0.saturating_sub(1), target_pos.1),
                ];

                for (x, y) in patterns {
                    self.fire_tiles.push(FireTile { x, y, duration: 2 });
                }
            }
        }
        if let Some(attacker) = self.players.iter_mut().find(|p| p.id == attacker_id) {
            if let Some(card) = attacker.cards.iter_mut().find(|c| c.id == card_id) {
                if card.is_signature {
                    if use_ability {
                        if let Some(CardEffect::Ability { cooldown, .. }) = card
                            .effects
                            .iter()
                            .find(|e| matches!(e, CardEffect::Ability { .. }))
                        {
                            card.cooldown = *cooldown;
                        }
                    } else {
                        card.cooldown = 1;
                    }
                } else {
                    attacker.cards.retain(|c| c.id != card_id);
                }
            }
        }

        let mut hit_landed = true;
        for effect in &card_effects {
            if let CardEffect::SkillCheck { threshold } = effect {
                if distance > 1 && rand::random_range(1..=6) < *threshold {
                    hit_landed = false;
                }
            }
        }
        if !hit_landed {
            return Ok(false);
        }

        for player in self.players.iter_mut() {
            let dx = (player.x as i16 - target_pos.0 as i16).abs();
            let dy = (player.y as i16 - target_pos.1 as i16).abs();

            let is_in_hit_zone = match card_name.as_str() {
                "Poison Bomb" => dx.max(dy) <= 1,
                _ => dx == 0 && dy == 0,
            };

            if is_in_hit_zone {
                for effect in &card_effects {
                    match effect {
                        CardEffect::Damage { power } => {
                            let final_dmg = (*power as f32 * damage_mod) as i32;
                            let initial_health = player.health;
                            if !shield_bypass {
                                if player.shield >= final_dmg {
                                    player.shield -= final_dmg;
                                } else {
                                    let overflow = final_dmg - player.shield;
                                    player.shield = 0;
                                    player.health = player.health.saturating_sub(overflow);
                                }
                            } else {
                                player.health = player.health.saturating_sub(final_dmg);
                            }
                            if has_lifesteal && player.id != attacker_id {
                                life_stolen += (initial_health - player.health).max(0);
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
                            player.shield = (player.shield + *value).min(10);
                        }
                        _ => {}
                    }
                }
            }
        }

        if life_stolen > 0 {
            if let Some(attacker) = self.players.iter_mut().find(|p| p.id == attacker_id) {
                attacker.health = (attacker.health + life_stolen).min(attacker.max_health);
            }
        }

        Ok(true)
    }

    pub fn initialise_deck(&mut self) {
        let mut new_deck: Vec<Card> = Vec::new();

        for _ in 0..2 {
            new_deck.push(Card::create_spiked_bat());
            new_deck.push(Card::create_poison_bomb());
        }
        for _ in 0..3 {
            new_deck.push(Card::create_bandage());
            new_deck.push(Card::create_antidote());
            new_deck.push(Card::create_sword());
        }
        for _ in 0..4 {
            new_deck.push(Card::create_stone());
            new_deck.push(Card::create_fangs());
            new_deck.push(Card::create_shield());
        }
        for _ in 0..5 {
            new_deck.push(Card::create_stick());
        }

        let mut rng = rand::rng();
        new_deck.shuffle(&mut rng);

        self.deck = new_deck;
    }
}
