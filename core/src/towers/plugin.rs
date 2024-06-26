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
            super::systems::init_towers_in_range,
            super::systems::compute_towers_in_range_on_range_update,
            super::systems::compute_towers_in_range_on_position_update,
            super::attacks::init_speed_buff_target,
            super::attacks::apply_speed_buff,

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
                stat_systems.chain(),
                super::systems::update_attack_energy,
            )
                .in_set(TowerUpdateSystems)
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                super::attacks::apply_basic_attack,
                super::attacks::apply_aoe_attack,
            )
                .in_set(TowerAttackSystems)
                .chain(),
        )
        .add_event::<super::events::TowerClickEvent>()
        .add_event::<super::attacks::BasicAttackEvent>()
        .add_event::<super::attacks::AoeAttackEvent>();

        app.add_systems(
            Update,
            (
                super::systems::render_model,
                super::systems::render_projectile_start,
                super::systems::render_projectile_end,
                super::systems::render_event_handlers,
                super::attacks::render_basic_attack,
                super::attacks::render_aoe_attack_start,
                super::attacks::render_aoe_attack_end,
            )
                .in_set(TowerRenderSystems),
        )
        .add_systems(
            Update,
            // GUI can trigger tower creation at any time so this needs to run every frame
            stat_systems.chain().in_set(TowerRenderSystems),
        );
    }
}
