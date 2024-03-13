use bevy::prelude::*;

use crate::{
    gui::console, path::Path, timers::tick_timer::TickTimer,
};

use super::components::{
    UnitModel, UnitSpawnTick, UnitStatus, UnitStatusTypes,
};

pub fn init_units_for_round(
    mut commands: Commands,
    paths: Query<(Entity, &Path)>,
) {
    for (path, _) in paths.iter() {
        super::basic_unit::spawn(&mut commands, path, 0);
    }
}

pub fn spawn_pending_units(
    mut commands: Commands,
    units: Query<(Entity, &UnitStatus, &UnitSpawnTick)>,
    tick_timer: Res<TickTimer>,
) {
    let to_spawn = units.iter().filter(|(_, status, spawn_tick)| {
        matches!(status.0, UnitStatusTypes::PRESPAWN)
            && spawn_tick.0 <= tick_timer.0
    });

    for (entity, _, spawn_tick) in to_spawn {
        let is_late = spawn_tick.0 < tick_timer.0;
        if is_late {
            console::warn(
                format!(
                    "Unit spawned late. Expected {} but got {}",
                    spawn_tick.0, tick_timer.0
                )
                .as_str(),
            )
        }

        commands
            .entity(entity)
            .insert(UnitStatus(UnitStatusTypes::ALIVE));
    }
}

pub fn render_status_change(
    units: Query<(&UnitStatus, &UnitModel), Changed<UnitStatus>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_query: Query<&Handle<StandardMaterial>>,
) {
    for (status, model) in units.iter() {
        let is_alive = matches!(status.0, UnitStatusTypes::ALIVE);
        let opacity = if is_alive { 1.0 } else { 0.0 };

        let handle = materials_query.get_mut(model.0).unwrap();
        let mat = materials.get_mut(handle).unwrap();
        mat.base_color.set_a(opacity);
    }
}

pub fn move_units() {}
