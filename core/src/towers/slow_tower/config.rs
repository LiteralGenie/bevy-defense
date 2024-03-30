use crate::towers::config::TowerConfig;

pub const CONFIG: TowerConfig = TowerConfig {
    id: 2,

    damage: 75,
    size: 2,
    speed: 7,
    range_radius: 4,

    spawn: super::spawn,
    spawn_model: super::spawn_model,
};
