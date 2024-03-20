use super::{
    components::{
        BaseDamage, BaseRangeRadius, BasicRangeType, EffectiveDamage,
        EffectiveRangeRadius, Projectile, TowerModel, TowerPosition,
    },
    events::TowerClickEvent,
    matchers::match_size,
};
use crate::{
    animation::components::InterpolateTranslation,
    gui::console,
    scenario::Scenario,
    timers::tick_timer::TickTimer,
    units::components::{
        UnitDist, UnitPathId, UnitStatus, UnitStatusTypes,
    },
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::collections::{HashMap, HashSet};

// Units binned by dist (from start of path)
#[derive(Resource)]
pub struct UnitsByDist(
    pub HashMap<u8, HashMap<u16, HashSet<Entity>>>,
);

pub fn index_units_by_dist(
    mut commands: Commands,
    scenario: Res<Scenario>,
    query: Query<(Entity, &UnitStatus, &UnitPathId, &UnitDist)>,
) {
    // Init bins
    let mut by_path = HashMap::new();

    for path in scenario.paths.values() {
        let mut by_dist: HashMap<u16, HashSet<Entity>> =
            HashMap::new();

        for idx in 0..path.points.iter().len() + 1 {
            by_dist.insert(idx as u16, HashSet::new());
        }

        by_path.insert(path.id, by_dist);
    }

    // Populate bins
    for (entity, status, id_path, dist) in query.iter() {
        if !matches!(status.0, UnitStatusTypes::ALIVE) {
            continue;
        }

        let path_bin = by_path.get_mut(&id_path.0).unwrap();
        let dist_bin = path_bin.get_mut(&dist.0).unwrap();
        dist_bin.insert(entity);
    }

    commands.insert_resource(UnitsByDist(by_path));
}

// @todo: Projectiles have no movement prediction
//        They only move towards the unit's position at time of spawn
//        So if the tower is far away, the projectile arrives too late -- behind the unit
pub fn render_attack_start(
    projectile_query: Query<(Entity, &Projectile), Added<Projectile>>,
    unit_query: Query<(&UnitPathId, &UnitDist)>,
    model_query: Query<&Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
    tick_timer: Res<TickTimer>,
) {
    const TRAVEL_SPEED: f32 = 1.25;

    for (entity, p) in projectile_query.iter() {
        let model = model_query.get(p.model).unwrap();

        let unit_pos = {
            let (unit_path, unit_dist) =
                unit_query.get(p.unit).unwrap();
            let path = scenario.paths.get(&unit_path.0).unwrap();
            path.points.get(unit_dist.0 as usize).unwrap()
        };
        let unit_pos = Vec3::new(
            unit_pos.pos.0 as f32,
            model.translation.y,
            unit_pos.pos.1 as f32,
        );

        let dist = (model.translation - unit_pos).length();
        let travel_ticks = (dist / TRAVEL_SPEED) as u32;

        commands.entity(entity).insert(InterpolateTranslation::new(
            p.model,
            model.translation,
            unit_pos,
            tick_timer.0,
            tick_timer.0 + travel_ticks,
        ));
    }
}

pub fn render_attack_end(
    query: Query<(Entity, &Projectile)>,
    mut done: RemovedComponents<InterpolateTranslation>,
    mut commands: Commands,
) {
    for entity in done.read() {
        if let Ok((entity, p)) = query.get(entity) {
            // Despawn projectile model
            commands.entity(p.model).despawn();

            // Despawn projectile
            commands.entity(entity).despawn();
        }
    }
}

pub fn render_event_handlers(
    query: Query<(Entity, &TowerModel), Added<TowerModel>>,
    mut commands: Commands,
) {
    for (entity, model) in query.iter() {
        commands.entity(model.0).insert((
            PickableBundle::default(),
            On::<Pointer<Click>>::run(
                move |mut writer: EventWriter<TowerClickEvent>| {
                    writer.send(TowerClickEvent(entity.clone()));
                },
            ),
        ));
    }
}

pub fn compute_effective_range(
    query: Query<
        (Entity, &BaseRangeRadius),
        Changed<BaseRangeRadius>,
    >,
    mut commands: Commands,
) {
    for (entity, base_radius) in query.iter() {
        let effective_radius = EffectiveRangeRadius(base_radius.0);
        commands.entity(entity).insert(effective_radius);
    }
}

pub fn compute_basic_range(
    query: Query<
        (Entity, &EffectiveRangeRadius, &TowerPosition),
        (With<BasicRangeType>, Changed<EffectiveRangeRadius>),
    >,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    for (entity, effective_radius, pos) in query.iter() {
        let range = BasicRangeType::create(
            effective_radius.0,
            pos.top_left,
            match_size(0),
            &scenario,
        );
        commands.entity(entity).insert(range);
    }
}

pub fn compute_effective_damage(
    query: Query<(Entity, &BaseDamage), Changed<BaseDamage>>,
    mut commands: Commands,
) {
    for (entity, base_damage) in query.iter() {
        let effective_damage = EffectiveDamage(base_damage.0);
        commands.entity(entity).insert(effective_damage);
    }
}
