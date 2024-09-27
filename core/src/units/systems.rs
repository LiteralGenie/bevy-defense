use std::collections::HashSet;

use super::{
    components::{
        BaseSpeed, EffectiveSpeed, UnitMaterial, UnitModel,
        UnitModelMaterials, UnitPosition, UnitSpawnTick, UnitStatus,
        UnitStatusTypes,
    },
    events::UnitDamageEvent,
    speed_trail_unit::SpeedBuff,
};
use crate::{
    animation::components::InterpolateTranslation,
    components::DoNotRender,
    gui::console::{self},
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

        let mut transform = models.get_mut(model.root).unwrap();
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
                    visibility_query.get_mut(model.root).unwrap();

                *visibility = Visibility::Hidden;
            }
            UnitStatusTypes::ALIVE => {
                let mut visibility =
                    visibility_query.get_mut(model.root).unwrap();

                *visibility = Visibility::Inherited;
            }
            UnitStatusTypes::DEAD => {
                commands.entity(model.root).despawn_recursive();
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
    mut transform_query: Query<&mut Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    for (entity, pos, model) in units.iter() {
        // Point defined by pos.dist
        let path = &scenario.paths[&pos.id_path];
        let point = path.points.get(pos.dist as usize).unwrap();

        // Point defined by pos.dist + 1
        let next_point =
            path.points.get((pos.dist + 1) as usize).unwrap_or(point);

        // Interpolate between the two based on accumulator value
        let (target_point, rotation) = {
            let start = point.pos;
            let end = next_point.pos;

            let diff_x = end.0 - start.0;
            let diff_y = end.1 - start.1;
            let diff = (diff_x, diff_y);

            let frac = pos.acc as f32 / 100.0;
            let scaled_diff =
                (diff.0 as f32 * frac, diff.1 as f32 * frac);

            let target_point = (
                start.0 as f32 + scaled_diff.0,
                start.1 as f32 + scaled_diff.1,
            );

            let rotation = {
                if diff_x > 0 {
                    0
                } else if diff_x < 0 {
                    180
                } else if diff_y > 0 {
                    -90
                } else {
                    90
                }
            };

            (target_point, rotation)
        };

        let mut base_transform =
            transform_query.get_mut(model.base).unwrap();

        base_transform.rotation =
            Quat::from_rotation_y((rotation as f32).to_radians());

        let root_transform =
            transform_query.get_mut(model.root).unwrap();

        let end = Vec3::new(
            target_point.0,
            root_transform.translation.y,
            target_point.1,
        );

        commands.entity(entity).insert(InterpolateTranslation::new(
            model.root,
            1,
            root_transform.translation,
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

// @todo: Move health bar update render here (ie wait until projectiles land)
pub fn render_unit_damage(
    mut reader: EventReader<UnitDamageEvent>,
    unit_query: Query<&UnitModelMaterials>,
    mut commands: Commands,
) {
    for ev in reader.read() {
        let mats = unit_query.get(ev.unit).unwrap();

        for unit_mat in mats.materials.iter() {
            let Some(mut unit) = commands.get_entity(unit_mat.entity)
            else {
                continue;
            };

            unit.insert(unit_mat.damage.clone());
        }
    }
}

/// Initialize UnitModelMaterials, which tracks...
///   - the original material
///   - the material for indicating damage (red tint of original)
pub fn render_unit_materials(
    units: Query<(Entity, &UnitModel), Without<UnitModelMaterials>>,
    mat_query: Query<&Handle<StandardMaterial>>,
    children_query: Query<&Children>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, model) in units.iter() {
        let mats =
            find_materials(&model.base, &mat_query, &children_query);

        if mats.len() == 0 {
            // Model isn't done loading
            continue;
        }

        let materials =
            Vec::from_iter(mats.into_iter().map(|(e, handle)| {
                let mat = materials.get(&handle).unwrap();

                let base_color = add_srgba(
                    mat.base_color.to_srgba(),
                    0.2,
                    -0.2,
                    -0.2,
                    Some(0.5),
                );

                // Based on https://github.com/aevyrie/bevy_mod_picking/blob/d8464161c5a499358d2816861d961078cbe01d1f/examples/gltf.rs#L55
                let damage_tint = StandardMaterial {
                    base_color,
                    ..mat.to_owned()
                };

                let damage_tint_handle = materials.add(damage_tint);

                UnitMaterial {
                    entity: e,
                    initial: handle,
                    damage: damage_tint_handle.clone(),
                }
            }));

        commands
            .entity(entity)
            .insert(UnitModelMaterials { materials });
    }
}

fn find_materials(
    entity: &Entity,
    mat_query: &Query<&Handle<StandardMaterial>>,
    children_query: &Query<&Children>,
) -> Vec<(Entity, Handle<StandardMaterial>)> {
    let mut to_check = vec![entity];
    let mut results = vec![];

    while to_check.len() > 0 {
        let e = to_check.pop().unwrap();
        if let Ok(handle) = mat_query.get(*e) {
            results.push((*e, handle.clone()));
        }

        if let Ok(children) = children_query.get(*e) {
            to_check.extend(children);
        }
    }

    return results;
}

fn add_srgba(
    base: Srgba,
    r: f32,
    g: f32,
    b: f32,
    a_override: Option<f32>,
) -> Color {
    Color::srgba(
        (base.red + r).clamp(0., 1.),
        (base.green + g).clamp(0., 1.),
        (base.blue + b).clamp(0., 1.),
        a_override.unwrap_or(base.alpha),
    )
}
