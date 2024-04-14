use crate::towers::components::{TowerPosition, TowerRange};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct TowersInRange(HashSet<Entity>);

pub fn init_towers_in_range(
    towers: Query<Entity, Without<TowersInRange>>,
    mut commands: Commands,
) {
    for entity in towers.iter() {
        commands
            .entity(entity)
            .insert(TowersInRange(HashSet::new()));
    }
}

// When range is updated, recalculate towers in range
pub fn compute_towers_in_range_on_range_update(
    mut updates: Query<
        (Entity, &TowerRange, &mut TowersInRange),
        Changed<TowerRange>,
    >,
    towers: Query<(Entity, &TowerPosition)>,
) {
    for (entity, range, mut in_range) in updates.iter_mut() {
        for (e_other, pos_other) in towers.iter() {
            if e_other == entity {
                continue;
            }

            if range.points.is_disjoint(&pos_other.coords) {
                in_range.0.remove(&e_other);
            } else {
                in_range.0.insert(e_other);
            }
        }
    }
}

// When position is updated, recalculate other towers' TowersInRange
pub fn compute_towers_in_range_on_position_update(
    updates: Query<(Entity, &TowerPosition), Changed<TowerPosition>>,
    mut towers: Query<(Entity, &TowerRange, &mut TowersInRange)>,
) {
    for (entity, range, mut in_range) in towers.iter_mut() {
        for (e_update, pos_update) in updates.iter() {
            if entity == e_update {
                continue;
            }

            if pos_update.coords.is_disjoint(&range.points) {
                in_range.0.remove(&e_update);
            } else {
                in_range.0.insert(e_update);
            }
        }
    }
}
