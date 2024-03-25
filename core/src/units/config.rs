use bevy::prelude::*;

pub struct UnitConfig {
    pub spawn: fn(&mut Commands, u8, u32),
}

pub fn match_config(id: u16) -> UnitConfig {
    match id {
        0 => super::basic_unit::CONFIG,
        1 => super::tank_unit::CONFIG,
        2 => super::speed_trail_unit::CONFIG,
        _ => panic!("Invalid unit id {}", id),
    }
}
