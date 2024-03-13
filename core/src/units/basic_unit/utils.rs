use super::super::components::{
    UnitDist, UnitHealth, UnitMarker, UnitPathId, UnitSpawnTick,
    UnitStatus, UnitStatusTypes,
};
use crate::gui::console;
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, id_path: u8, tick: u32) {
    commands.spawn((
        UnitMarker,
        UnitStatus(UnitStatusTypes::PRESPAWN),
        UnitDist(0),
        UnitHealth(100),
        UnitPathId(id_path),
        UnitSpawnTick(tick),
        super::Marker,
    ));
}
