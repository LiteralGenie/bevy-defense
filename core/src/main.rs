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
        // Load game into canvas#game-canvas
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    ..default()
                }),
                ..default()
            })
        )
        // Init game state
        .add_systems(Startup, (
            map::spawn_map,
            camera::spawn_camera,
            path::spawn_paths,
            player::spawn_players,
            timer::spawn_timer,
        ))
        // Update our custom timer's state
        .add_systems(Update, timer::update_timer)
        // Update GUI
        .add_systems(Update, gui::rx::handle_gui_requests)
        .run();
}
