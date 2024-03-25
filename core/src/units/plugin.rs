use crate::{scenario::spawn_scenario, states::GamePhase};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitUpdateSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitRenderSystems;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            super::speed_trail_unit::init_buff_map
                .after(spawn_scenario),
        )
        .add_systems(
            OnEnter(GamePhase::BUILD),
            (super::systems::init_units_for_round,),
        )
        .add_systems(
            FixedUpdate,
            (
                super::speed_trail_unit::update_speed_buff_duration,
                super::speed_trail_unit::spawn_speed_buff,
                super::speed_trail_unit::apply_speed_buff,
                super::systems::compute_effective_speed,
                super::systems::spawn_pending_units,
                super::systems::move_units,
            )
                .in_set(UnitUpdateSystems)
                .chain(),
        );

        app.add_systems(
            Update,
            (
                super::systems::render_status_change,
                super::health_bar::render_health_bar,
                super::basic_unit::render,
                super::tank_unit::render,
                super::speed_trail_unit::render,
            )
                .in_set(UnitRenderSystems),
        )
        .add_systems(
            FixedUpdate,
            (super::systems::render_movement_start,)
                .after(UnitUpdateSystems),
        );
    }
}
