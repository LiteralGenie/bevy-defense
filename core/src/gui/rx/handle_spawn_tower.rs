use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    gui::{console, utils::window_to_world_coords},
    towers,
};

pub fn handle_spawn_tower(world: &mut World, cursor_pos: Vec2) {
    let mut state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
        Query<(&Camera, &GlobalTransform)>,
    )> = SystemState::new(world);

    let (commands, meshes, materials, camera_query) =
        state.get_mut(world);

    let (camera, camera_transform) = camera_query.single();
    let world_pos =
        window_to_world_coords(camera, camera_transform, cursor_pos);

    towers::basic_tower::spawn_model(
        commands,
        meshes,
        materials,
        Vec2::new(world_pos.x.round(), world_pos.z.round()),
    );
    state.apply(world);
}
