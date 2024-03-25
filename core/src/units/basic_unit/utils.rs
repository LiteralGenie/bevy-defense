use crate::{gui::console, units::components::BaseUnitBundle};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, id_path: u8, tick: u32) {
    commands.spawn((
        BaseUnitBundle::new(id_path, tick, 100, 100),
        super::Marker,
    ));
}
