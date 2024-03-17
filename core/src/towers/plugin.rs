use crate::{states::GamePhase, units};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerUpdateSystems;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                // Pre-sort units by each targeting criteria (dist, health, etc)
                super::systems::index_units_by_dist,
                // Update tower stats
                super::systems::compute_effective_damage,
                super::systems::compute_effective_range,
                super::systems::compute_basic_range,
                // Apply damage to units
                super::attacks::apply_basic_attack,
            )
                .chain()
                .after(units::plugin::UnitUpdateSystems)
                .run_if(in_state(GamePhase::COMBAT)),
        )
        .add_event::<super::attacks::BasicAttackEvent>()
        .add_event::<super::events::TowerClickEvent>();

        app.add_systems(
            Update,
            (
                super::basic_tower::render,
                super::systems::render_attacks,
                super::systems::render_event_handlers,
                super::attacks::render_basic_attack,
            ),
        );
    }
}
