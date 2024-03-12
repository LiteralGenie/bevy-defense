use bevy::prelude::*;

use crate::path::Path;

pub fn init_units_for_round(
    mut commands: Commands,
    paths: Query<&Path>,
) {
    for path in paths.iter() {
        super::basic_unit::spawn(&mut commands, path.id);
    }
}

pub fn move_units() {}
