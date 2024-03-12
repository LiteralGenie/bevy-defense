use bevy::prelude::*;

use crate::{gui::console, states::GamePhase};

pub fn handle_start_round(world: &mut World) {
    let phase =
        world.get_resource::<State<GamePhase>>().unwrap().get();

    match phase {
        GamePhase::BUILD => {
            let next_phase = &mut world
                .get_resource_mut::<NextState<GamePhase>>()
                .unwrap();
            next_phase.set(GamePhase::COMBAT)
        }
        _ => console::error(
            "start_round request sent outside of build phase",
        ),
    }
}
