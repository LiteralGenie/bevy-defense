use bevy::prelude::*;

use crate::{
    animation::components::InterpolateTranslation,
    components::DoNotRender,
    gui::console,
    scenario::{Direction, Scenario},
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
        let dir = &path.segments.get(0).unwrap().dir;

        let mut transform = models.get_mut(model.0).unwrap();
        let translation = &mut transform.translation;
        translation.x = point.pos.0 as f32;
        translation.z = point.pos.1 as f32;

        // Offset a bit for the sake of having an initial movement animation
        match dir {
            Direction::Up => {
                translation.z -= 1.0;
            }
            Direction::Down => {
                translation.z += 1.0;
            }
            Direction::Right => {
                translation.x -= 1.0;
            }
            Direction::Left => {
                translation.x += 1.0;
            }
        }
    }
}

pub fn render_status_change(
    units: Query<
        (Entity, &UnitStatus, &UnitModel),
        Or<(Changed<UnitStatus>, Added<UnitModel>)>,
    >,
    mut visibility_query: Query<&mut Visibility>,
    mut commands: Commands,
) {
    for (entity, status, model) in units.iter() {
        match status.0 {
            UnitStatusTypes::PRESPAWN => {
                let mut visibility =
                    visibility_query.get_mut(model.0).unwrap();

                *visibility = Visibility::Hidden;
            }
            UnitStatusTypes::ALIVE => {
                let mut visibility =
                    visibility_query.get_mut(model.0).unwrap();

                *visibility = Visibility::Inherited;
            }
            UnitStatusTypes::DEAD => {
                commands.entity(model.0).despawn_recursive();
                commands.entity(entity).remove::<UnitModel>();
                commands.entity(entity).insert(DoNotRender);
            }
        }
    }
}

pub fn render_movement_start(
    units: Query<
        (Entity, &UnitPathId, &UnitDist, &UnitModel),
        Changed<UnitDist>,
    >,
    models: Query<&Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
    tick: Res<TickTimer>,
) {
    for (entity, path_id, dist, model) in units.iter() {
        let translation = models.get(model.0).unwrap().translation;

        let path = &scenario.paths[&path_id.0];
        let point = path.points.get(dist.0 as usize).unwrap();
        let end = Vec3::new(
            point.pos.0 as f32,
            translation.y,
            point.pos.1 as f32,
        );

        // @fixme: It seems this system runs after the animation system
        //         that handles this component, causing a no-op
        //         Fix this when refactoring into explicit system ordering
        commands.entity(entity).insert(InterpolateTranslation::new(
            model.0,
            translation,
            end,
            tick.0 + 1,
            tick.0 + 2,
        ));
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
