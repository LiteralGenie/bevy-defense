use crate::towers::config::{
    RangeConfig, RangeTypeConfig, TowerConfig, TowerVariantConfig,
};

pub const CONFIG: TowerConfig = TowerConfig {
    id: 3,

    size: 2,
    range: RangeConfig {
        radius: 5,
        variant: RangeTypeConfig::Basic,
    },

    offense: None,
    variant: TowerVariantConfig::SpeedBuff(1.35),

    render_model: super::render_model,
};
