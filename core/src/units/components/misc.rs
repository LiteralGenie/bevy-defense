use crate::units::config::match_config;

use super::BaseSpeed;
use bevy::prelude::*;
pub enum UnitStatusTypes {
    PRESPAWN,
    ALIVE,
    DEAD,
}

#[derive(Component)]
pub struct UnitStatus(pub UnitStatusTypes);

#[derive(Component)]
pub struct UnitMarker;

#[derive(Component)]
pub struct UnitPosition {
    pub id_path: u8,
    pub dist: u16,
    pub acc: u16,
}

#[derive(Component)]
pub struct UnitHealth(pub u32);

#[derive(Component)]
pub struct UnitHealthMax(pub u32);

#[derive(Component)]
pub struct UnitSpawnTick(pub u32);

#[derive(Bundle)]
pub struct BaseUnitBundle {
    marker: UnitMarker,
    position: UnitPosition,
    health: UnitHealth,
    health_max: UnitHealthMax,
    spawn_tick: UnitSpawnTick,
    speed: BaseSpeed,
    status: UnitStatus,
}

impl BaseUnitBundle {
    pub fn new(id_unit: u16, id_path: u8, spawn_tick: u32) -> Self {
        let cfg = match_config(id_unit);

        Self {
            marker: UnitMarker,
            health: UnitHealth(cfg.health_max),
            health_max: UnitHealthMax(cfg.health_max),
            position: UnitPosition {
                id_path,
                dist: 0,
                acc: 0,
            },
            spawn_tick: UnitSpawnTick(spawn_tick),
            speed: BaseSpeed(cfg.speed),
            status: UnitStatus(UnitStatusTypes::PRESPAWN),
        }
    }
}
