use std::collections::HashSet;

use super::{
    components::{
        BaseSpeed, EffectiveSpeed, UnitModel, UnitPosition,
        UnitSpawnTick, UnitStatus, UnitStatusTypes,
    },
    speed_trail_unit::SpeedBuff,
};
use crate::{
    animation::components::InterpolateTranslation,
    components::DoNotRender,
    gui::console,
    scenario::Scenario,
    timers::{round_timer::RoundTimer, tick_timer::TickTimer},
};
use bevy::prelude::*;

pub fn init_units_for_round(
    mut commands: Commands,
    scenario: Res<Scenario>,
    round_timer: Res<RoundTimer>,
) {
    let wave = &scenario.waves[round_timer.round as usize];
    for enemy in wave.enemies.iter() {
        let cfg = super::config::match_config(enemy.id_unit);
        (cfg.spawn)(
            &mut commands,
            enemy.id_path,
            round_timer.start_tick + enemy.delay,
        );
    }
}

pub fn spawn_pending_units(
    mut commands: Commands,
    units: Query<(
        Entity,
        &UnitStatus,
        &UnitSpawnTick,
        &UnitPosition,
        &UnitModel,
    )>,
    tick_timer: Res<TickTimer>,
    scenario: Res<Scenario>,
    mut models: Query<&mut Transform>,
) {
    let to_spawn =
        units.iter().filter(|(_, status, spawn_tick, _, _)| {
            matches!(status.0, UnitStatusTypes::PRESPAWN)
                && spawn_tick.0 <= tick_timer.0
        });

    for (entity, _, spawn_tick, pos, model) in to_spawn {
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
        let path = &scenario.paths[&pos.id_path];
        let point = path.points.get(pos.dist as usize).unwrap();

        let mut transform = models.get_mut(model.0).unwrap();
        let translation = &mut transform.translation;
        translation.x = point.pos.0 as f32;
        translation.z = point.pos.1 as f32;
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
        (Entity, &UnitPosition, &UnitModel),
        Changed<UnitPosition>,
    >,
    models: Query<&Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    for (entity, pos, model) in units.iter() {
        let translation = models.get(model.0).unwrap().translation;

        // Point defined by pos.dist
        let path = &scenario.paths[&pos.id_path];
        let point = path.points.get(pos.dist as usize).unwrap();

        // Point defined by pos.dist + 1
        let next_point =
            path.points.get((pos.dist + 1) as usize).unwrap_or(point);

        // Interpolate between the two based on accumulator value
        let target_point = {
            let start = point.pos;
            let end = next_point.pos;
            let diff = (end.0 - start.0, end.1 - start.1);

            let frac = pos.acc as f32 / 100.0;
            let scaled_diff =
                (diff.0 as f32 * frac, diff.1 as f32 * frac);

            (
                start.0 as f32 + scaled_diff.0,
                start.1 as f32 + scaled_diff.1,
            )
        };

        let end =
            Vec3::new(target_point.0, translation.y, target_point.1);

        commands.entity(entity).insert(InterpolateTranslation::new(
            model.0,
            1,
            translation,
            end,
        ));
    }
}

pub fn move_units(
    mut units: Query<(
        &mut UnitPosition,
        &mut UnitStatus,
        &EffectiveSpeed,
    )>,
    scenario: Res<Scenario>,
) {
    let mut alive = units.iter_mut().filter(|(_, status, _)| {
        matches!(status.0, UnitStatusTypes::ALIVE)
    });

    for (mut pos, mut status, speed) in &mut alive {
        // Update position
        pos.acc += speed.0;
        while pos.acc >= 100 {
            pos.acc -= 100;
            pos.dist += 1;
        }

        // If at end of path, kill unit
        let len = scenario.paths[&pos.id_path].points.len() as u16;
        pos.dist = pos.dist.min(len - 1);
        if pos.dist == len - 1 {
            status.0 = UnitStatusTypes::DEAD;
        }
    }
}

pub fn compute_effective_speed(
    info_query: Query<(&BaseSpeed, Option<&SpeedBuff>)>,
    changed: Query<
        Entity,
        Or<(Changed<BaseSpeed>, Changed<SpeedBuff>)>,
    >,
    mut unbuffed: RemovedComponents<SpeedBuff>,
    mut commands: Commands,
) {
    let to_check: HashSet<Entity> =
        HashSet::from_iter(changed.iter().chain(unbuffed.read()));

    for entity in to_check {
        let (base, buff) = {
            match info_query.get(entity) {
                Ok(res) => res,
                Err(_) => continue,
            }
        };

        let mut update = base.0 as f64;

        if let Some(buff) = buff {
            update *= buff.0;
        }

        let update = update.min(100.0) as u16;

        commands.entity(entity).insert(EffectiveSpeed(update));
    }
}
