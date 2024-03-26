use crate::towers::{
    attacks,
    components::{BaseTowerBundle, BasicRangeType},
};
use bevy::prelude::*;

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(size, 1.0, size)),
            material: materials.add(Color::rgb(0.25, 0.25, 0.25)),
            transform: Transform::from_xyz(offset, 0.5, -offset),
            ..default()
        })
        .id();

    commands.entity(container).push_children(&[model]);

    container
}

pub fn spawn(world: &mut World, pos: (i16, i16)) {
    world.spawn((
        BaseTowerBundle::new(super::CONFIG.id, pos),
        super::Marker,
        BasicRangeType,
        attacks::BasicAttack,
    ));
}
