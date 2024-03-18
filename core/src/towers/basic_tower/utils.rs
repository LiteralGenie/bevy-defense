use crate::towers::{
    attacks,
    components::{BaseTowerBundle, BasicRangeType},
};
use bevy::prelude::*;

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) -> Entity {
    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.z),
        ..default()
    };
    let entity = commands.spawn(model).id();

    entity
}

pub fn spawn(world: &mut World, pos: (i16, i16)) {
    world.spawn((
        BaseTowerBundle::new(pos, 10, 4),
        super::Marker,
        BasicRangeType,
        attacks::BasicAttack,
    ));
}
