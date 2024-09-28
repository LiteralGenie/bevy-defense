use bevy::prelude::*;

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &mut Res<AssetServer>,
    top_left: Vec3,
) -> Entity {
    let size = super::CONFIG.size as f32;
    let offset = (size - 1.0) / 2.0;

    let container = commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(
                top_left.x, 0.0, top_left.z,
            ),
            ..default()
        })
        .id();

    let model = commands
        .spawn(SceneBundle {
            scene: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("railgun_turret/custom_railgun.glb"),
            ),
            transform: Transform::from_translation(Vec3::new(
                0.45, 0.0, -0.5,
            ))
            .with_scale(Vec3::splat(1.3)),
            ..default()
        })
        .id();

    commands.entity(container).push_children(&[model]);

    container
}
