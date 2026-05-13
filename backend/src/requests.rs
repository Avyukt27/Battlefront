use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveRequest {
    pub player_id: u32,
    pub target_x: u8,
    pub target_y: u8,
}

#[derive(Deserialize)]
pub struct UseCardRequest {
    pub card_id: String,
    pub attacker_id: u32,
    pub target_pos: (u8, u8),
}

#[derive(Deserialize)]
pub struct PlayerRequest {
    pub player_id: u32,
}
