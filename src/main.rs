use bevy::prelude::*;
mod map;
mod camera;
mod units;
mod path;

fn debug(map: Res<map::Map>) {
    units::basic_unit::BasicUnitBundle {
        dist: units::components::UnitDist(0),
        hp: units::components::UnitHealth(0),
    };
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_systems(Startup, (map::load_map, camera::load_camera, path::load_paths))
        .add_systems(Update, debug)
        .run();
}
