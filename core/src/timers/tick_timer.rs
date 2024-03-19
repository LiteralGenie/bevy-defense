use bevy::ecs::system::{Commands, ResMut, Resource};

pub const TICK_FREQUENCY_HZ: f64 = 5.0;

#[derive(Resource)]
pub struct TickTimer(pub u32);

impl TickTimer {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(TickTimer(0));
}

pub fn update_timer(mut timer: ResMut<TickTimer>) {
    timer.increment();
}
