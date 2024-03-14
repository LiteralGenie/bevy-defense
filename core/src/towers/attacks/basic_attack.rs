use bevy::prelude::*;

use crate::{
    scenario::Scenario,
    towers::{
        components::{TowerPriority, TowerRange},
        systems::UnitsByDist,
    },
    units::components::{
        UnitDist, UnitHealth, UnitPathId, UnitStatus, UnitStatusTypes,
    },
};

use super::utils::{filter_targets_by_dist, find_target};

#[derive(Component)]
pub struct BasicAttack {
    pub damage: u32,
}

pub fn apply_basic_attack(
    query: Query<(&BasicAttack, &TowerRange, &TowerPriority)>,
    targets_by_dist: Res<UnitsByDist>,
    mut info_query: Query<(&UnitPathId, &UnitDist, &mut UnitHealth)>,
    scenario: Res<Scenario>,
    mut status_query: Query<&mut UnitStatus>,
) {
    for (attack, range, priority) in query.iter() {
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
        health.0 = health.0.saturating_sub(attack.damage);

        if health.0 == 0 {
            let mut status = status_query.get_mut(target).unwrap();
            status.0 = UnitStatusTypes::DEAD;
        }
    }
}
