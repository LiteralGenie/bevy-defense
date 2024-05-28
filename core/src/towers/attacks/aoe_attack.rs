use super::utils::{filter_targets_by_dist, find_target};
use crate::{
    animation::components::{
        InterpolateScale, InterpolateTranslation,
    },
    scenario::Scenario,
    timers::tick_timer::TICK_FREQUENCY_HZ,
    towers::{
        components::{
            EffectiveDamage, Projectile, TowerAttackEnergy,
            TowerPosition, TowerPriority, TowerRange,
        },
        systems::UnitsByDist,
    },
    units::components::{
        UnitHealth, UnitPosition, UnitStatus, UnitStatusTypes,
    },
};
use bevy::prelude::*;

#[derive(Component)]
pub struct AoeAttack(pub u16);

#[derive(Component)]
pub struct AoeModelMarker;

#[derive(Event)]
pub struct AoeAttackEvent {
    pub tower: Entity,
    pub units: Vec<Entity>,
    pub radius: u16,
    pub id_path: u8,
    pub dist: u16,
}

pub fn apply_aoe_attack(
    mut query: Query<(
        Entity,
        &AoeAttack,
        &mut TowerAttackEnergy,
        &EffectiveDamage,
        &TowerRange,
        &TowerPriority,
    )>,
    targets_by_dist: Res<UnitsByDist>,
    pos_query: Query<&UnitPosition>,
    mut health_query: Query<&mut UnitHealth>,
    scenario: Res<Scenario>,
    mut status_query: Query<&mut UnitStatus>,
    mut events: EventWriter<AoeAttackEvent>,
) {
    for (entity, attack, mut energy, damage, range, priority) in
        query.iter_mut()
    {
        if energy.charges == 0 {
            continue;
        } else {
            energy.charges = 0;
        }

        // @todo: It's a little wasteful to grab all the candidates if the priority is distance-based
        //        eg if we want the first unit....
        //          we can iterate over the tower range in descending order
        //          check the bin for each point
        //          and return the first unit found
        let candidates =
            filter_targets_by_dist(&targets_by_dist, range);

        let primary_target = match find_target(
            &priority.0,
            &candidates,
            &pos_query,
            &scenario,
        ) {
            Some(val) => val,
            None => continue,
        };

        let primary_pos = pos_query.get(primary_target).unwrap();

        let targets = candidates
            .clone()
            .into_iter()
            .filter(|entity| {
                let pos = pos_query.get(*entity).unwrap();
                (primary_pos.dist).abs_diff(pos.dist) <= attack.0
            })
            .collect::<Vec<_>>();

        for tgt in targets.iter() {
            let mut health = health_query.get_mut(*tgt).unwrap();
            health.0 = health.0.saturating_sub(damage.0);

            if health.0 == 0 {
                let mut status = status_query.get_mut(*tgt).unwrap();
                status.0 = UnitStatusTypes::DEAD;
            }
        }

        events.send(AoeAttackEvent {
            tower: entity,
            units: targets,
            radius: attack.0,
            id_path: primary_pos.id_path,
            dist: primary_pos.dist,
        });
    }
}

pub fn render_aoe_attack_start(
    mut reader: EventReader<AoeAttackEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scenario: Res<Scenario>,
) {
    for ev in reader.read() {
        let path = scenario.paths.get(&ev.id_path).unwrap();
        let center = path.points.get(ev.dist as usize).unwrap();

        let mut model = commands.spawn((
            PbrBundle {
                mesh: meshes
                    .add(Capsule3d::new(ev.radius as f32, 0.01)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.5, 0.0, 0.0, 0.25),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3::new(
                        center.pos.0 as f32,
                        0.75,
                        center.pos.1 as f32,
                    ),
                    scale: Vec3::splat(0.05),
                    ..default()
                },
                ..default()
            },
            AoeModelMarker,
        ));

        model.insert(InterpolateScale::new(
            model.id(),
            (0.75 * TICK_FREQUENCY_HZ) as u32,
            0.05,
            1.0,
        ));
    }
}

pub fn render_aoe_attack_end(
    query: Query<Entity, With<AoeModelMarker>>,
    mut done: RemovedComponents<InterpolateScale>,
    mut commands: Commands,
) {
    for entity in done.read() {
        if let Ok(entity) = query.get(entity) {
            commands.entity(entity).despawn();
        }
    }
}
