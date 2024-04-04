use crate::towers::config::{
    AttackTypeConfig, OffenseConfig, RangeConfig, RangeTypeConfig,
    TowerConfig, TowerVariantConfig,
};

pub const CONFIG: TowerConfig = TowerConfig {
    id: 2,

    size: 2,
    range: RangeConfig {
        radius: 4,
        variant: RangeTypeConfig::Basic,
    },

    offense: Some(OffenseConfig {
        attack: AttackTypeConfig::Basic,
        damage: 75,
        speed: 7,
    }),
    variant: TowerVariantConfig::Basic,

    spawn_model: super::spawn_model,
};
