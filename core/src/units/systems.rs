use bevy::prelude::*;

use crate::{
    gui::console, path::Path, timers::tick_timer::TickTimer,
};

use super::components::{
    UnitDist, UnitModel, UnitPath, UnitSpawnTick, UnitStatus,
    UnitStatusTypes,
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
    units: Query<(
        Entity,
        &UnitStatus,
        &UnitSpawnTick,
        &UnitPath,
        &UnitDist,
        &UnitModel,
    )>,
    tick_timer: Res<TickTimer>,
    paths: Query<&Path>,
    mut models: Query<&mut Transform>,
) {
    let to_spawn =
        units.iter().filter(|(_, status, spawn_tick, _, _, _)| {
            matches!(status.0, UnitStatusTypes::PRESPAWN)
                && spawn_tick.0 <= tick_timer.0
        });

    for (entity, _, spawn_tick, path_id, dist, model) in to_spawn {
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

        // Change unit status from PRESPAWN -> ALIVE
        commands
            .entity(entity)
            .insert(UnitStatus(UnitStatusTypes::ALIVE));

        // Move unit to start of path
        let path = paths.get(path_id.0).unwrap();
        let point = path.points.get(dist.0 as usize).unwrap();

        let mut transform = models.get_mut(model.0).unwrap();
        let translation = &mut transform.translation;
        translation.x = point.pos.0 as f32;
        translation.z = point.pos.1 as f32;
    }
}

pub fn render_status_change(
    units: Query<
        (&UnitStatus, &UnitModel),
        Or<(Changed<UnitStatus>, Added<UnitModel>)>,
    >,
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

pub fn render_movement(
    units: Query<
        (&UnitPath, &UnitDist, &UnitModel),
        (With<UnitModel>, Or<(Changed<UnitDist>, Added<UnitModel>)>),
    >,
    mut models: Query<&mut Transform>,
    paths: Query<&Path>,
) {
    for (path_id, dist, model) in units.iter() {
        let path = paths.get(path_id.0).unwrap();
        let point = path.points.get(dist.0 as usize).unwrap();

        let mut transform = models.get_mut(model.0).unwrap();
        let translation = &mut transform.translation;
        translation.x = point.pos.0 as f32;
        translation.z = point.pos.1 as f32;
    }
}

pub fn move_units(
    mut units: Query<(&UnitPath, &mut UnitDist, &mut UnitStatus)>,
    paths: Query<&Path>,
) {
    let mut alive = units.iter_mut().filter(|(_, _, status)| {
        matches!(status.0, UnitStatusTypes::ALIVE)
    });

    for (path_id, mut dist, mut status) in &mut alive {
        // Update position
        dist.0 += 1;

        // If at end of path, kill unit
        let path = paths.get(path_id.0).unwrap();
        if dist.0 as usize == path.points.len() - 1 {
            status.0 = UnitStatusTypes::DEAD;
        }
    }
}
