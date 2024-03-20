use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimerUpdateSystems;

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
                .in_set(TimerUpdateSystems),
        );
    }
}
