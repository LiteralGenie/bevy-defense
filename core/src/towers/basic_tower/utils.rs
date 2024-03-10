use bevy::{ecs::system::SystemState, prelude::*};

use crate::towers::components::{
    TowerMarker, TowerModel, TowerPosition,
};

pub fn spawn(world: &mut World, pos: Vec3) {
    let model = super::spawn_model(world, pos, 1.0);

    let mut state: SystemState<Commands> = SystemState::new(world);
    let mut commands = state.get_mut(world);

    commands.spawn((
        TowerMarker,
        TowerPosition {
            x: pos.x as i16,
            z: pos.z as i16,
        },
        TowerModel(model),
        super::Marker,
    ));

    state.apply(world);
}
