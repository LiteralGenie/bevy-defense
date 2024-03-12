use bevy::ecs::system::{Commands, ResMut, Resource};

#[derive(Resource)]
pub struct RoundTimer {
    pub round: u32,
}

impl RoundTimer {
    pub fn next(&mut self) {
        self.round += 1;
    }
}

impl Default for RoundTimer {
    fn default() -> RoundTimer {
        RoundTimer { round: 0 }
    }
}

pub fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(RoundTimer {
        ..Default::default()
    });
}

pub fn update_timer(mut timer: ResMut<RoundTimer>) {
    timer.next();
}
