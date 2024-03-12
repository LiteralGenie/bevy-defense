use bevy::prelude::*;
use states::GamePhase;
mod camera;
mod gui;
mod map;
mod path;
mod player;
mod render_plugin;
mod states;
mod timers;
mod towers;
mod units;

fn main() {
    let mut app = App::new();

    app
        // Load game into canvas#game-canvas
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game-canvas".into()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Init game state
        .init_state::<GamePhase>()
        .add_systems(
            Startup,
            (
                map::spawn_map,
                camera::spawn_camera,
                path::spawn_paths,
                player::spawn_players,
                timers::tick_timer::spawn_timer,
                timers::round_timer::spawn_timer,
                // @temp
                units::systems::init_units_for_round,
            ),
        )
        // Build phase
        .add_systems(
            OnEnter(GamePhase::BUILD),
            (units::systems::init_units_for_round,),
        )
        // Combat phase
        .insert_resource(Time::<Fixed>::from_hz(5.0))
        .add_systems(
            FixedUpdate,
            (
                timers::tick_timer::update_timer,
                units::systems::move_units,
            )
                .run_if(in_state(GamePhase::COMBAT)),
        );

    if cfg!(server) {
        // @todo
    } else {
        app
            // Draw stuff to screen
            .add_plugins(render_plugin::RenderPlugin)
            // Send state updates to gui
            .add_plugins(gui::tx::plugin::TxPlugin)
            // Read requests from gui
            .add_plugins(gui::rx::plugin::RxPlugin);
    }

    app.run();
}
