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
