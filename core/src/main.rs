use bevy::prelude::*;
use states::GamePhase;
mod camera;
mod gui;
mod map;
mod player;
mod scenario;
mod states;
mod timers;
mod towers;
mod units;
use bevy_mod_picking::{
    picking_core::PickingPluginsSettings, prelude::*,
};

fn main() {
    let mut app = App::new();

    app
        // Load game into canvas#game-canvas
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            scenario::plugin::ScenarioPlugin,
            timers::plugin::TimersPlugin,
            units::plugin::UnitsPlugin,
            towers::plugin::TowersPlugin,
        ))
        // Init game state
        .init_state::<GamePhase>()
        .add_systems(
            Startup,
            (
                map::spawn_map,
                camera::spawn_camera,
                player::spawn_players,
            ),
        )
        .insert_resource(Time::<Fixed>::from_hz(5.0));

    if cfg!(server) {
        // @todo
    } else {
        app
            // Emit events on model click / hover / etc
            .add_plugins(
                DefaultPickingPlugins
                    .build()
                    // Disable default highlighting effects
                    .disable::<DefaultHighlightingPlugin>(),
            )
            // Send state updates to gui
            .add_plugins(gui::tx::plugin::TxPlugin)
            // Read requests from gui
            .add_plugins(gui::rx::plugin::RxPlugin);
    }

    app.run();
}
