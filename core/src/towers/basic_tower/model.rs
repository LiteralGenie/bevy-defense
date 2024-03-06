use bevy::prelude::*;

pub fn spawn_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pos: Vec2,
) -> Entity {
    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb_u8(255, 0, 0)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.y),
        ..default()
    };

    commands.spawn(model).id()
}
