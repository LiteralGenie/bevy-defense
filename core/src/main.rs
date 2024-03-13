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
            scenario::plugin::PathPlugin,
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
                timers::tick_timer::spawn_timer,
                timers::round_timer::spawn_timer,
            ),
        )
        // Build phase
        // .add_systems(OnEnter(GamePhase::BUILD), ())
        // Combat phase
        .insert_resource(Time::<Fixed>::from_hz(5.0))
        .add_systems(
            FixedUpdate,
            (timers::tick_timer::update_timer,)
                .run_if(in_state(GamePhase::COMBAT)),
        );

    if cfg!(server) {
        // @todo
    } else {
        app
            // Send state updates to gui
            .add_plugins(gui::tx::plugin::TxPlugin)
            // Read requests from gui
            .add_plugins(gui::rx::plugin::RxPlugin);
    }

    app.run();
}
