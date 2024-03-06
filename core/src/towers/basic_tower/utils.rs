use bevy::{ecs::system::SystemState, prelude::*};

use crate::towers::components::{
    TowerMarker, TowerModel, TowerPosition,
};

use super::{spawn_model, Marker};

pub fn spawn(world: &mut World, pos: Vec3) {
    let model = spawn_model(world, pos, 255);

    let mut state: SystemState<Commands> = SystemState::new(world);
    let mut commands = state.get_mut(world);

    commands.spawn((
        TowerMarker,
        TowerPosition {
            x: pos.x as i16,
            z: pos.z as i16,
        },
        TowerModel(model),
        Marker,
    ));

    state.apply(world);
}
