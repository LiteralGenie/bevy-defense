use super::utils::{filter_targets_by_dist, find_target};
use crate::{
    scenario::Scenario,
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
pub struct BasicAttack;

#[derive(Event)]
pub struct BasicAttackEvent {
    pub tower: Entity,
    pub unit: Entity,
}

pub fn apply_basic_attack(
    mut query: Query<
        (
            Entity,
            &mut TowerAttackEnergy,
            &EffectiveDamage,
            &TowerRange,
            &TowerPriority,
        ),
        With<BasicAttack>,
    >,
    targets_by_dist: Res<UnitsByDist>,
    pos_query: Query<&UnitPosition>,
    mut health_query: Query<&mut UnitHealth>,
    scenario: Res<Scenario>,
    mut status_query: Query<&mut UnitStatus>,
    mut events: EventWriter<BasicAttackEvent>,
) {
    for (entity, mut energy, damage, range, priority) in
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

        let target = match find_target(
            &priority.0,
            candidates,
            &pos_query,
            &scenario,
        ) {
            Some(val) => val,
            None => continue,
        };

        let mut health = health_query.get_mut(target).unwrap();
        health.0 = health.0.saturating_sub(damage.0);

        if health.0 == 0 {
            let mut status = status_query.get_mut(target).unwrap();
            status.0 = UnitStatusTypes::DEAD;
        }

        events.send(BasicAttackEvent {
            tower: entity,
            unit: target,
        });
    }
}

pub fn render_basic_attack(
    mut reader: EventReader<BasicAttackEvent>,
    tower_query: Query<&TowerPosition>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in reader.read() {
        let pos = tower_query.get(ev.tower).unwrap();
        let offset = (pos.size as f32 - 1.0) / 2.0;

        let model = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.25, 0.25)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_xyz(
                    pos.top_left.0 as f32 + offset,
                    0.75,
                    pos.top_left.1 as f32 - offset,
                ),
                ..default()
            })
            .id();

        commands.spawn(Projectile {
            unit: ev.unit,
            model,
        });
    }
}
