use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerUpdateSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerAttackSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerRenderSystems;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        let stat_systems = (
            super::systems::compute_effective_attack_speed,
            super::systems::compute_effective_damage,
            super::systems::compute_effective_range,
            super::systems::compute_basic_range,
        );

        app.add_systems(
            FixedUpdate,
            (
                // Pre-sort units by each targeting criteria (dist, health, etc)
                super::systems::index_units_by_dist,
                // Update tower stats
                stat_systems,
                super::systems::update_attack_energy,
            )
                .in_set(TowerUpdateSystems)
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                // Apply damage to units
                super::attacks::apply_basic_attack,
            )
                .in_set(TowerAttackSystems)
                .chain(),
        )
        .add_event::<super::attacks::BasicAttackEvent>()
        .add_event::<super::events::TowerClickEvent>();

        app.add_systems(
            Update,
            (
                super::systems::render_model,
                super::systems::render_attack_start,
                super::systems::render_attack_end,
                super::systems::render_event_handlers,
                super::attacks::render_basic_attack,
            )
                .in_set(TowerRenderSystems),
        )
        .add_systems(
            Update,
            // GUI can trigger tower creation at any time so this needs to run every frame
            stat_systems.in_set(TowerRenderSystems),
        );
    }
}
