use bevy::prelude::*;

pub struct RenderPlugin;

// @todo: if server build, disable rendering
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::towers::render_plugin::RenderPlugin);
        app.add_plugins(crate::units::render_plugin::RenderPlugin);
    }
}
