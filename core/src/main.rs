use bevy::prelude::*;
mod camera;
mod gui;
mod map;
mod path;
mod player;
mod timer;
mod towers;
mod units;

fn main() {
    App::new()
        // Load game into canvas#game-canvas
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game-canvas".into()),
                ..default()
            }),
            ..default()
        }))
        // Init game state
        .add_systems(
            Startup,
            (
                map::spawn_map,
                camera::spawn_camera,
                path::spawn_paths,
                player::spawn_players,
                timer::spawn_timer,
            ),
        )
        // Run game logic on fixed timestep
        .insert_resource(Time::<Fixed>::from_hz(5.0))
        .add_systems(FixedUpdate, (timer::update_timer,))
        // Send state updates to gui
        .add_plugins(gui::tx::plugin::TxPlugin)
        // Read requests from gui
        .add_plugins(gui::rx::plugin::RxPlugin)
        .run();
}
