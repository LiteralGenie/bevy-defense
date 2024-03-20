use crate::towers::{
    attacks,
    components::{BaseTowerBundle, BasicRangeType},
    matchers::match_size,
};
use bevy::prelude::*;

pub const ID: u16 = 0;

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    top_left: Vec3,
) -> Entity {
    let size = match_size(ID) as f32;
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
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(offset, 0.0, -offset),
            ..default()
        })
        .id();

    commands.entity(container).push_children(&[model]);

    container
}

pub fn spawn(world: &mut World, pos: (i16, i16)) {
    world.spawn((
        BaseTowerBundle::new(pos, 10, match_size(ID), 5),
        super::Marker,
        BasicRangeType,
        attacks::BasicAttack,
    ));
}
