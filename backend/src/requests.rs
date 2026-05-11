use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveRequest {
    pub player_id: u32,
    pub target_x: u8,
    pub target_y: u8,
}

#[derive(Deserialize)]
pub struct JoinRequest {
    pub name: String,
    pub class: String,
}
