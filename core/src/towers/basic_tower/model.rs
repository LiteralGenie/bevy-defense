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
