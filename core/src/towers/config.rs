// @todo: replace with compile-time hashmap?

use super::basic_tower::CONFIG;

pub struct TowerConfig {
    pub damage: u32,
    pub size: u8,
    pub range_radius: u8,
}

pub fn match_config(id: u16) -> TowerConfig {
    match id {
        0 => CONFIG,
        _ => panic!("Invalid tower id {}", id),
    }
}
