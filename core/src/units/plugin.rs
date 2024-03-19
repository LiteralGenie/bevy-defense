use crate::{states::GamePhase, timers::tick_timer};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitUpdateSystems;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GamePhase::BUILD),
            (super::systems::init_units_for_round,),
        )
        .add_systems(
            FixedUpdate,
            (
                super::systems::move_units,
                super::systems::spawn_pending_units,
            )
                .in_set(UnitUpdateSystems)
                .chain()
                .run_if(in_state(GamePhase::COMBAT))
                .after(tick_timer::update_timer),
        );

        app.add_systems(
            Update,
            (
                super::systems::render_status_change,
                super::health_bar::render_health_bar,
                super::basic_unit::render,
                super::tank_unit::render,
            ),
        )
        .add_systems(
            FixedUpdate,
            (super::systems::render_movement_start,),
        );
    }
}
