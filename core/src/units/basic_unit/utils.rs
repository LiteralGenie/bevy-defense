use bevy::prelude::*;

use crate::{
    gui::console,
    units::components::{
        UnitDist, UnitHealth, UnitMarker, UnitPathId, UnitSpawnTick,
        UnitStatus, UnitStatusTypes,
    },
};

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
