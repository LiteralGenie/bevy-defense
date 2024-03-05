use bevy::ecs::system::{ Commands, Res, ResMut, Resource };

#[derive(Resource)]
pub struct TickTimer {
    pub tick: u32,
}

impl TickTimer {
    pub fn tick(&mut self) {
        self.tick += 1;
    }
}

impl Default for TickTimer {
    fn default() -> TickTimer {
        return TickTimer {
            tick: 0,
        };
    }
}

pub fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(TickTimer { ..Default::default() });
}

pub fn update_timer(mut timer: ResMut<TickTimer>) {
    timer.tick();
}
