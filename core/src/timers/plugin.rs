use crate::states::GamePhase;
use bevy::prelude::*;

pub struct TimersPlugin;

impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                super::tick_timer::spawn_timer,
                super::round_timer::spawn_timer,
            ),
        )
        .add_systems(
            FixedUpdate,
            (super::tick_timer::update_timer,)
                .run_if(in_state(GamePhase::COMBAT)),
        );
    }
}
