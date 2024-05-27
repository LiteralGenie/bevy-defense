use crate::towers::config::{
    AttackTypeConfig, OffenseConfig, RangeConfig, RangeTypeConfig,
    TowerConfig, TowerVariantConfig,
};

pub const CONFIG: TowerConfig = TowerConfig {
    id: 1,

    size: 2,
    range: RangeConfig {
        radius: 6,
        variant: RangeTypeConfig::Basic,
    },

    offense: Some(OffenseConfig {
        attack: AttackTypeConfig::Basic,
        damage: 11,
        speed: 35,
    }),
    variant: TowerVariantConfig::Basic,

    spawn_model: super::spawn_model,
};
