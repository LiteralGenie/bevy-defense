use crate::towers::config::TowerConfig;

pub const ID: u16 = 1;

pub const CONFIG: TowerConfig = TowerConfig {
    damage: 10,
    size: 2,
    speed: 40,
    range_radius: 6,

    spawn: super::spawn,
};
