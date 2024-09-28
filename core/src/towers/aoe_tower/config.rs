use crate::towers::config::{
    AttackTypeConfig, OffenseConfig, RangeConfig, RangeTypeConfig,
    TowerConfig, TowerVariantConfig,
};

pub const CONFIG: TowerConfig = TowerConfig {
    id: 4,

    size: 2,
    range: RangeConfig {
        radius: 7,
        variant: RangeTypeConfig::Basic,
    },

    offense: Some(OffenseConfig {
        attack: AttackTypeConfig::Aoe(4),
        damage: 25,
        speed: 5,
    }),
    variant: TowerVariantConfig::Basic,

    render_model: super::render_model,
};
