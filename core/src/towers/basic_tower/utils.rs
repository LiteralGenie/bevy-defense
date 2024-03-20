use crate::towers::{
    attacks,
    components::{BaseTowerBundle, BasicRangeType},
};
use bevy::prelude::*;

pub const SIZE_RADIUS: u8 = 2;

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    top_left: Vec3,
) -> Entity {
    const OFFSET: f32 = (SIZE_RADIUS as f32 - 1.0) / 2.0;

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
            mesh: meshes.add(Cuboid::new(
                SIZE_RADIUS as f32,
                1.0,
                SIZE_RADIUS as f32,
            )),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(OFFSET, 0.0, OFFSET),
            ..default()
        })
        .id();

    commands.entity(container).push_children(&[model]);

    container
}

pub fn spawn(world: &mut World, pos: (i16, i16)) {
    world.spawn((
        BaseTowerBundle::new(pos, 10, SIZE_RADIUS, 5),
        super::Marker,
        BasicRangeType,
        attacks::BasicAttack,
    ));
}
