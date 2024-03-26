use crate::towers::config::TowerConfig;

pub const CONFIG: TowerConfig = TowerConfig {
    id: 0,

    damage: 30,
    size: 2,
    speed: 15,
    range_radius: 5,

    spawn: super::spawn,
    spawn_model: super::spawn_model,
};
