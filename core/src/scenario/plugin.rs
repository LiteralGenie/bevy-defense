use bevy::prelude::*;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, super::systems::spawn_scenario);

        app.add_systems(PostStartup, super::systems::render_paths);
    }
}
