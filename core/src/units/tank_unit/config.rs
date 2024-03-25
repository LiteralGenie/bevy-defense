use crate::units::config::UnitConfig;

pub const ID: u16 = 1;

pub const CONFIG: UnitConfig = UnitConfig {
    health_max: 300,
    speed: 20,
    spawn: super::utils::spawn,
};
