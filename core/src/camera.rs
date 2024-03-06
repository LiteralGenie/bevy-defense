use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1500.0,
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z),
            ..Default::default()
        },
        Camera,
    ));
}
