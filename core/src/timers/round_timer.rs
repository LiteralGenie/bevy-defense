use super::tick_timer::TickTimer;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundTimer {
    pub round: u16,
    pub start_tick: u32,
}

impl RoundTimer {
    pub fn increment(&mut self, tick: u32) {
        self.round += 1;
        self.start_tick = tick;
    }
}

pub fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(RoundTimer {
        start_tick: 0,
        round: 0,
    });
}

pub fn update_timer(
    mut timer: ResMut<RoundTimer>,
    mut tick_timer: Res<TickTimer>,
) {
    timer.increment(tick_timer.0);
}
