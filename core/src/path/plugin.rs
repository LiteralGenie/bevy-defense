use bevy::prelude::*;

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, super::systems::spawn_paths);

        app.add_systems(PostStartup, super::systems::render_paths);
    }
}
