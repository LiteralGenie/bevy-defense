use bevy::ecs::system::{Commands, ResMut, Resource};

#[derive(Resource)]
pub struct RoundTimer(pub u32);

impl RoundTimer {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(RoundTimer(0));
}

pub fn update_timer(mut timer: ResMut<RoundTimer>) {
    timer.increment();
}
