use bevy::prelude::*;

pub struct TowerConfig {
    pub id: u16,

    pub damage: u32,
    pub size: u8,
    // Attack speed bounded by [0, 100]
    pub speed: u8,
    pub range_radius: u8,

    pub spawn: fn(&mut World, (i16, i16)),
    pub spawn_model: fn(
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<StandardMaterial>>,
        Vec3,
    ) -> Entity,
}

// @todo: replace with compile-time hashmap?
pub fn match_config(id: u16) -> TowerConfig {
    match id {
        0 => super::basic_tower::CONFIG,
        1 => super::fast_tower::CONFIG,
        _ => panic!("Invalid tower id {}", id),
    }
}

pub const TOWER_CONFIGS: &[TowerConfig] =
    &[super::basic_tower::CONFIG, super::fast_tower::CONFIG];
