use bevy::prelude::*;

use crate::units;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerAttackSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetIndexSystems;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app
            // Pre-sort units by each targeting criteria (dist, health, etc)
            .add_systems(
                FixedUpdate,
                (super::systems::index_units_by_dist,)
                    .in_set(TargetIndexSystems)
                    .after(units::plugin::UnitUpdateSystems),
            )
            // Apply damage to units
            .add_systems(
                FixedUpdate,
                (super::attacks::basic_attack::apply_basic_attack,)
                    .in_set(TowerAttackSystems)
                    .after(TargetIndexSystems),
            );

        app.add_systems(Update, (super::basic_tower::render,));
    }
}
