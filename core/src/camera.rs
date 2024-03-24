use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1500.0,
    });

    commands.spawn((
        Camera3dBundle {
            // Top-down camera
            transform: Transform::from_xyz(0.0, 10.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(30.0),
                ..default()
            }
            .into(),
            ..Default::default()
        },
        Camera,
    ));
}
