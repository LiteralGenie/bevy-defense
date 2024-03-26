use crate::towers::config::TowerConfig;

pub const CONFIG: TowerConfig = TowerConfig {
    id: 1,

    damage: 11,
    size: 2,
    speed: 35,
    range_radius: 6,

    spawn: super::spawn,
    spawn_model: super::spawn_model,
};
