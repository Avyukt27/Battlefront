use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveRequest {
    pub player_id: u32,
    pub target_x: u8,
    pub target_y: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCardRequest {
    pub card_id: String,
    pub attacker_id: u32,
    pub target_pos: (u8, u8),
    pub use_ability: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRequest {
    pub player_id: u32,
}
