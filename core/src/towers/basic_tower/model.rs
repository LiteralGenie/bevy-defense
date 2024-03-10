use bevy::{ecs::system::SystemState, prelude::*};

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

    let (mut commands, meshes, materials) = state.get_mut(world);

    let model = init_model(meshes, materials, opacity, pos);
    let entity = commands.spawn(model).id();

    state.apply(world);

    entity
}

pub fn spawn_model_with_marker(
    world: &mut World,
    pos: Vec3,
    opacity: f32,
    bundle: impl Bundle,
) -> Entity {
    let mut state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )> = SystemState::new(world);

    let (mut commands, meshes, materials) = state.get_mut(world);

    let model = init_model(meshes, materials, opacity, pos);
    let entity = commands.spawn(model).insert(bundle).id();

    state.apply(world);

    entity
}

fn init_model(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    opacity: f32,
    pos: Vec3,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgba(1.0, 0.0, 0.0, opacity)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.z),
        ..default()
    }
}
