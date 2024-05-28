use crate::towers::config::{
    AttackTypeConfig, OffenseConfig, RangeConfig, RangeTypeConfig,
    TowerConfig, TowerVariantConfig,
};

pub const CONFIG: TowerConfig = TowerConfig {
    id: 4,

    size: 2,
    range: RangeConfig {
        radius: 4,
        variant: RangeTypeConfig::Basic,
    },

    offense: Some(OffenseConfig {
        attack: AttackTypeConfig::Aoe(3),
        damage: 25,
        speed: 5,
    }),
    variant: TowerVariantConfig::Basic,

    spawn_model: super::spawn_model,
};
