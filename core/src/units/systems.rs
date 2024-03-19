use bevy::prelude::*;

use crate::{
    gui::console,
    scenario::Scenario,
    timers::{round_timer::RoundTimer, tick_timer::TickTimer},
};

use super::components::{
    UnitDist, UnitModel, UnitPathId, UnitSpawnTick, UnitStatus,
    UnitStatusTypes,
};

pub fn init_units_for_round(
    mut commands: Commands,
    scenario: Res<Scenario>,
    round_timer: Res<RoundTimer>,
) {
    let wave = &scenario.waves[round_timer.round as usize];
    for enemy in wave.enemies.iter() {
        match enemy.id_unit {
            0 => {
                super::basic_unit::spawn(
                    &mut commands,
                    enemy.id_path,
                    round_timer.start_tick + enemy.delay,
                );
            }
            1 => {
                super::tank_unit::spawn(
                    &mut commands,
                    enemy.id_path,
                    round_timer.start_tick + enemy.delay,
                );
            }
            _ => {
                panic!("Invalid unit id: {}", enemy.id_unit)
            }
        }
    }
}

pub fn spawn_pending_units(
    mut commands: Commands,
    units: Query<(
        Entity,
        &UnitStatus,
        &UnitSpawnTick,
        &UnitPathId,
        &UnitDist,
        &UnitModel,
    )>,
    tick_timer: Res<TickTimer>,
    scenario: Res<Scenario>,
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
        let path = &scenario.paths[&path_id.0];
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
    mut visibility_query: Query<&mut Visibility>,
) {
    for (status, model) in units.iter() {
        let is_alive = matches!(status.0, UnitStatusTypes::ALIVE);
        let update = if is_alive {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        let mut visibility =
            visibility_query.get_mut(model.0).unwrap();

        *visibility = update;
    }
}

pub fn render_movement(
    units: Query<
        (&UnitPathId, &UnitDist, &UnitModel),
        (With<UnitModel>, Or<(Changed<UnitDist>, Added<UnitModel>)>),
    >,
    mut models: Query<&mut Transform>,
    scenario: Res<Scenario>,
) {
    for (path_id, dist, model) in units.iter() {
        let path = &scenario.paths[&path_id.0];
        let point = path.points.get(dist.0 as usize).unwrap();

        let mut transform = models.get_mut(model.0).unwrap();
        let translation = &mut transform.translation;
        translation.x = point.pos.0 as f32;
        translation.z = point.pos.1 as f32;
    }
}

pub fn move_units(
    mut units: Query<(&UnitPathId, &mut UnitDist, &mut UnitStatus)>,
    scenario: Res<Scenario>,
) {
    let mut alive = units.iter_mut().filter(|(_, _, status)| {
        matches!(status.0, UnitStatusTypes::ALIVE)
    });

    for (path_id, mut dist, mut status) in &mut alive {
        // Update position
        dist.0 += 1;

        // If at end of path, kill unit
        let path = &scenario.paths[&path_id.0];
        if dist.0 as usize == path.points.len() - 1 {
            status.0 = UnitStatusTypes::DEAD;
        }
    }
}
