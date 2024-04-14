use crate::towers::config::{
    AttackTypeConfig, OffenseConfig, RangeConfig, RangeTypeConfig,
    TowerConfig, TowerVariantConfig,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

pub const CONFIG: TowerConfig = TowerConfig {
    id: 0,

    size: 2,
    range: RangeConfig {
        radius: 5,
        variant: RangeTypeConfig::Basic,
    },

    offense: Some(OffenseConfig {
        attack: AttackTypeConfig::Basic,
        damage: 30,
        speed: 15,
    }),
    variant: TowerVariantConfig::Basic,

    spawn_model: super::spawn_model,
};
