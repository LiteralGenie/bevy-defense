use super::{
    components::{
        BaseAttackSpeed, BaseDamage, BaseRangeRadius, BasicRangeType,
        EffectiveAttackSpeed, EffectiveDamage, EffectiveRangeRadius,
        Projectile, TowerAttackEnergy, TowerMarker, TowerModel,
        TowerPosition,
    },
    config::match_config,
    events::TowerClickEvent,
};
use crate::{
    animation::components::InterpolateTranslation,
    gui::console,
    scenario::Scenario,
    units::components::{UnitPosition, UnitStatus, UnitStatusTypes},
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::collections::{HashMap, HashSet};

// Units binned by dist (from start of path)
#[derive(Resource)]
pub struct UnitsByDist(
    pub HashMap<u8, HashMap<u16, HashSet<Entity>>>,
);

pub fn render_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    towers: Query<
        (Entity, &TowerMarker, &TowerPosition),
        Without<TowerModel>,
    >,
) {
    for (entity, id, pos) in towers.iter() {
        let cfg = match_config(id.0);

        let model = (cfg.spawn_model)(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(
                pos.top_left.0 as f32,
                0.0,
                pos.top_left.1 as f32,
            ),
        );

        commands.entity(entity).insert(TowerModel(model));
    }
}

// @todo: It's a little wasteful to run this system if no towers have enough charge to attack this tick
//        Consider benchmarking the performance impact and if necessary, make this lazy somehow
pub fn index_units_by_dist(
    mut commands: Commands,
    scenario: Res<Scenario>,
    query: Query<(Entity, &UnitStatus, &UnitPosition)>,
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
    for (entity, status, pos) in query.iter() {
        if !matches!(status.0, UnitStatusTypes::ALIVE) {
            continue;
        }

        let path_bin = by_path.get_mut(&pos.id_path).unwrap();
        let dist_bin = path_bin.get_mut(&pos.dist).unwrap();
        dist_bin.insert(entity);
    }

    commands.insert_resource(UnitsByDist(by_path));
}

pub fn update_attack_energy(
    mut query: Query<(&mut TowerAttackEnergy, &EffectiveAttackSpeed)>,
) {
    for (mut energy, speed) in query.iter_mut() {
        energy.acc += speed.0;

        while energy.acc >= 100 {
            energy.acc -= 100;
            energy.charges += 1;
        }
    }
}

// @todo: Projectiles have no movement prediction
//        They only move towards the unit's position at time of spawn
//        So if the tower is far away, the projectile arrives too late -- behind the unit
pub fn render_attack_start(
    projectile_query: Query<(Entity, &Projectile), Added<Projectile>>,
    pos_query: Query<&UnitPosition>,
    model_query: Query<&Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    const TRAVEL_SPEED: f32 = 0.5;

    for (entity, p) in projectile_query.iter() {
        let model = model_query.get(p.model).unwrap();

        let coord = {
            let pos = pos_query.get(p.unit).unwrap();
            let path = scenario.paths.get(&pos.id_path).unwrap();
            path.points.get(pos.dist as usize).unwrap()
        };
        let unit_pos = Vec3::new(
            coord.pos.0 as f32,
            model.translation.y,
            coord.pos.1 as f32,
        );

        let dist = (model.translation - unit_pos).length();
        let travel_ticks = (dist / TRAVEL_SPEED) as u32;

        commands.entity(entity).insert(InterpolateTranslation::new(
            p.model,
            travel_ticks,
            model.translation,
            unit_pos,
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
        (Entity, &TowerMarker, &EffectiveRangeRadius, &TowerPosition),
        (With<BasicRangeType>, Changed<EffectiveRangeRadius>),
    >,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    for (entity, id, effective_radius, pos) in query.iter() {
        let range = BasicRangeType::create(
            effective_radius.0,
            pos.top_left,
            match_config(id.0).size,
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

pub fn compute_effective_attack_speed(
    info_query: Query<&BaseAttackSpeed>,
    changed: Query<Entity, Or<(Changed<BaseAttackSpeed>,)>>,
    mut commands: Commands,
) {
    let to_check: HashSet<Entity> =
        HashSet::from_iter(changed.iter());

    for entity in to_check {
        let base = {
            match info_query.get(entity) {
                Ok(res) => res,
                Err(_) => continue,
            }
        };

        let mut update = base.0 as f32;

        // Cap attack speed at once per tick
        update = update.min(100.0);

        commands
            .entity(entity)
            .insert(EffectiveAttackSpeed(update as u8));
    }
}
