use bevy::prelude::*;
mod gui;
mod map;
mod camera;
mod units;
mod path;
mod player;
mod timer;
mod towers;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, (
            map::spawn_map,
            camera::spawn_camera,
            path::spawn_paths,
            player::spawn_players,
            timer::spawn_timer,
        ))
        .add_systems(Update, timer::update_timer)
        .add_systems(Update, gui::rx::handle_gui_requests)
        .run();
}
