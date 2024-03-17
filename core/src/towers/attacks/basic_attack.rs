use bevy::prelude::*;

use crate::{
    scenario::Scenario,
    towers::{
        components::{
            EffectiveDamage, Projectile, TowerPosition,
            TowerPriority, TowerRange,
        },
        systems::UnitsByDist,
    },
    units::components::{
        UnitDist, UnitHealth, UnitPathId, UnitStatus, UnitStatusTypes,
    },
};

use super::utils::{filter_targets_by_dist, find_target};

#[derive(Component)]
pub struct BasicAttack;

#[derive(Event)]
pub struct BasicAttackEvent {
    pub tower: Entity,
    pub unit: Entity,
}

pub fn apply_basic_attack(
    query: Query<
        (Entity, &EffectiveDamage, &TowerRange, &TowerPriority),
        With<BasicAttack>,
    >,
    targets_by_dist: Res<UnitsByDist>,
    mut info_query: Query<(&UnitPathId, &UnitDist, &mut UnitHealth)>,
    scenario: Res<Scenario>,
    mut status_query: Query<&mut UnitStatus>,
    mut events: EventWriter<BasicAttackEvent>,
) {
    for (entity, damage, range, priority) in query.iter() {
        let candidates =
            filter_targets_by_dist(&targets_by_dist, range);

        let target = match find_target(
            &priority.0,
            candidates,
            &info_query,
            &scenario,
        ) {
            Some(val) => val,
            None => continue,
        };

        let (_, _, mut health) = info_query.get_mut(target).unwrap();
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
        let tower_pos = tower_query.get(ev.tower).unwrap();

        let model = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.25, 0.25)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_xyz(
                    tower_pos.x as f32,
                    0.5,
                    tower_pos.z as f32,
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
