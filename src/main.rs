use bevy::prelude::*;
mod map;
mod camera;

fn debug(map: Res<map::Map>) {}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_systems(Startup, (map::load_map, camera::load_camera))
        .add_systems(Update, debug)
        .run();
}
