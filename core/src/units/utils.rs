use bevy::prelude::*;

pub fn render_health_bar(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Rectangle::new(0.75, 0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    }
}
