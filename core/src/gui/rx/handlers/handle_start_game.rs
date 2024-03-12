use bevy::prelude::*;

use crate::{gui::console, states::GamePhase};

pub fn handle_start_game(world: &mut World) {
    let phase =
        world.get_resource::<State<GamePhase>>().unwrap().get();

    match phase {
        GamePhase::INIT => {
            let next_phase = &mut world
                .get_resource_mut::<NextState<GamePhase>>()
                .unwrap();
            next_phase.set(GamePhase::BUILD)
        }
        _ => console::error(
            "start_game request sent outside of init phase",
        ),
    }
}
