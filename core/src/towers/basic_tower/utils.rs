use super::super::components::{TowerMarker, TowerPosition};
use bevy::{ecs::system::SystemState, prelude::*};

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) -> Entity {
    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.z),
        ..default()
    };
    let entity = commands.spawn(model).id();

    entity
}

pub fn spawn(world: &mut World, pos: Vec3) {
    let mut state: SystemState<Commands> = SystemState::new(world);
    let mut commands = state.get_mut(world);

    commands.spawn((
        TowerMarker,
        TowerPosition {
            x: pos.x as i16,
            z: pos.z as i16,
        },
        super::Marker,
    ));

    state.apply(world);
}
