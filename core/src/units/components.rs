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
pub struct UnitDist(pub u16);

#[derive(Component)]
pub struct UnitHealth(pub u16);

#[derive(Component)]
pub struct UnitPathId(pub u8);

#[derive(Component)]
pub struct UnitSpawnTick(pub u32);

#[derive(Component)]
pub struct UnitModel(pub Entity);

#[derive(Bundle)]
pub struct BaseUnitBundle {
    marker: UnitMarker,
    dist: UnitDist,
    health: UnitHealth,
    id_path: UnitPathId,
    spawn_tick: UnitSpawnTick,
    status: UnitStatus,
}

impl BaseUnitBundle {
    pub fn new(health: u16, id_path: u8, spawn_tick: u32) -> Self {
        Self {
            marker: UnitMarker,
            dist: UnitDist(0),
            health: UnitHealth(health),
            id_path: UnitPathId(id_path),
            spawn_tick: UnitSpawnTick(spawn_tick),
            status: UnitStatus(UnitStatusTypes::PRESPAWN),
        }
    }
}
