use crate::units::config::UnitConfig;

pub const ID: u16 = 0;

pub const CONFIG: UnitConfig = UnitConfig {
    health_max: 100,
    speed: 30,
    spawn: super::utils::spawn,
};
