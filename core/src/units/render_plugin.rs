use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                super::systems::render_status_change,
                super::basic_unit::render_models,
                super::basic_unit::render_movement,
            ),
        );
    }
}
