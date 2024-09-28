use bevy::prelude::*;

use crate::states::GamePhase;

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
        _ => log::error!(
            "start_round request sent outside of build phase",
        ),
    }
}
