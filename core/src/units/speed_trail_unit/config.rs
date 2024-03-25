use crate::units::config::UnitConfig;

pub const ID: u16 = 2;

pub const CONFIG: UnitConfig = UnitConfig {
    health_max: 50,
    speed: 50,
    spawn: super::utils::spawn,
};

pub const BUFF_DURATION: u16 = 30;

pub const BUFF_MULTIPLIER: f64 = 1.5;
