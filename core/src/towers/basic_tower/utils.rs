use bevy::{ecs::system::SystemState, prelude::*};

use crate::towers::components::{
    TowerMarker, TowerModel, TowerPosition,
};

pub fn spawn_model(
    world: &mut World,
    pos: Vec3,
    opacity: f32,
) -> Entity {
    let mut state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )> = SystemState::new(world);

    let (mut commands, mut meshes, mut materials) =
        state.get_mut(world);

    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgba(1.0, 0.0, 0.0, opacity)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.z),
        ..default()
    };
    let entity = commands.spawn(model).id();

    state.apply(world);

    entity
}

pub fn spawn(world: &mut World, pos: Vec3) {
    let model = spawn_model(world, pos, 1.0);

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
