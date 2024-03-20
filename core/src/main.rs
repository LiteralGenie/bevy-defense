use bevy::prelude::*;
use states::GamePhase;
mod animation;
mod camera;
mod components;
mod gui;
mod map;
mod player;
mod scenario;
mod states;
mod timers;
mod towers;
mod units;
use bevy_mod_picking::prelude::*;

fn main() {
    let mut app = App::new();

    app
        // Load game into canvas#game-canvas
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game-canvas".into()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),))
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
        .insert_resource(Time::<Fixed>::from_hz(
            timers::tick_timer::TICK_FREQUENCY_HZ,
        ))
        // Order tick-based systems
        .configure_sets(
            FixedUpdate,
            (
                timers::plugin::TimerUpdateSystems,
                units::plugin::UnitUpdateSystems
                    .after(timers::plugin::TimerUpdateSystems),
                towers::plugin::TowerUpdateSystems
                    .after(units::plugin::UnitUpdateSystems),
                towers::plugin::TowerAttackSystems
                    .after(towers::plugin::TowerUpdateSystems),
            )
                .chain()
                .run_if(in_state(GamePhase::COMBAT)),
        )
        // Plugins should be added *after* their SystemSet(s) are configured
        .add_plugins((
            scenario::plugin::ScenarioPlugin,
            timers::plugin::TimersPlugin,
            units::plugin::UnitsPlugin,
            towers::plugin::TowersPlugin,
        ));

    if cfg!(server) {
        // @todo
    } else {
        app
            // Utility systems for rendering transforms that span multiple frames
            .add_plugins(animation::plugin::AnimationPlugin)
            // Emit events on model click / hover / etc
            .add_plugins(
                DefaultPickingPlugins
                    .build()
                    // Disable color changes on click / hover
                    .disable::<DefaultHighlightingPlugin>(),
            )
            // Send state updates to gui
            .add_plugins(gui::tx::plugin::TxPlugin)
            // Read requests from gui
            .add_plugins(gui::rx::plugin::RxPlugin);
    }

    app.run();
}
