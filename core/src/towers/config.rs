use bevy::prelude::*;

pub struct TowerConfig {
    pub id: u16,

    pub range: RangeConfig,
    pub size: u8,

    pub offense: Option<OffenseConfig>,
    pub variant: TowerVariantConfig,

    pub spawn_model: fn(
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<StandardMaterial>>,
        Vec3,
    ) -> Entity,
}

pub enum RangeTypeConfig {
    Basic,
}

pub struct RangeConfig {
    pub variant: RangeTypeConfig,
    pub radius: u8,
}

pub enum AttackTypeConfig {
    Basic,
    Aoe(u16),
}

pub struct OffenseConfig {
    pub attack: AttackTypeConfig,
    pub damage: u32,
    // Attack speed bounded by [0, 100], where 100 is 1 attack per tick
    pub speed: u8,
}

// Catch-all for any tower-specific properties
pub enum TowerVariantConfig {
    Basic,
    SpeedBuff(f64),
}

// @todo: replace with compile-time hashmap?
pub fn match_config(id: u16) -> TowerConfig {
    match id {
        0 => super::basic_tower::CONFIG,
        1 => super::fast_tower::CONFIG,
        2 => super::slow_tower::CONFIG,
        3 => super::speed_buff_tower::CONFIG,
        _ => panic!("Invalid tower id {}", id),
    }
}

pub const TOWER_CONFIGS: &[TowerConfig] = &[
    super::basic_tower::CONFIG,
    super::fast_tower::CONFIG,
    super::slow_tower::CONFIG,
    super::speed_buff_tower::CONFIG,
];
