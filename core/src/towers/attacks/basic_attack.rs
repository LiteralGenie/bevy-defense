use bevy::prelude::*;

use crate::{
    towers::{components::TowerPriority, systems::TargetsByDist},
    units::components::{UnitHealth, UnitStatus},
};

use super::utils::find_target;

#[derive(Component)]
pub struct BasicAttack {
    damage: u32,
}

pub fn apply_basic_attack(
    query: Query<(&BasicAttack, &TowerPriority)>,
    status_query: Query<&UnitStatus>,
    mut health_query: Query<&mut UnitHealth>,
    targets_by_dist: Res<TargetsByDist>,
) {
    for (attack, priority) in query.iter() {
        let target = match find_target(
            &priority.0,
            &targets_by_dist,
            &status_query,
        ) {
            Some(val) => val,
            None => continue,
        };

        let mut health = health_query.get_mut(target).unwrap();
        health.0 = health.0.saturating_sub(attack.damage);
    }
}
