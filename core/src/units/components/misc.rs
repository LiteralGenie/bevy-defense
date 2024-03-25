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

// @todo: move unit stats to config (same with tower)
//        these new fns should instead take an id and pull the data from the config matcher
impl BaseUnitBundle {
    pub fn new(
        id_path: u8,
        spawn_tick: u32,
        health: u32,
        speed: u16,
    ) -> Self {
        Self {
            marker: UnitMarker,
            health: UnitHealth(health),
            health_max: UnitHealthMax(health),
            position: UnitPosition {
                id_path,
                dist: 0,
                acc: 0,
            },
            spawn_tick: UnitSpawnTick(spawn_tick),
            speed: BaseSpeed(speed),
            status: UnitStatus(UnitStatusTypes::PRESPAWN),
        }
    }
}
