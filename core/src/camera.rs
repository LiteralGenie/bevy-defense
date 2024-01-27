use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn load_camera(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 40000.0,
            shadows_enabled: true,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-0.25, 20.0, 0.0)).looking_at(
                Vec3::ZERO,
                Vec3::Y
            ),
            ..Default::default()
        },
        Camera,
    ));
}
