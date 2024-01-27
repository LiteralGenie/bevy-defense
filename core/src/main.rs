use bevy::prelude::*;
mod gui;
mod map;
mod camera;
mod units;
mod path;
mod player;
mod timer;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, (
            map::load_map,
            camera::load_camera,
            path::load_paths,
            player::load_players,
            timer::load_timer,
        ))
        .add_systems(Update, timer::update_timer)
        .add_systems(Update, gui::rx::handle_gui_requests)
        .run();
}
