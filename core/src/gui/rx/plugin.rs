use bevy::prelude::*;

pub struct RxPlugin;

impl Plugin for RxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, super::router::handle_gui_requests);
    }
}
