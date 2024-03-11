use bevy::{ecs::system::SystemState, prelude::*};

use crate::towers::components::{
    TowerMarker, TowerModel, TowerPosition,
};

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    opacity: f32,
) -> Entity {
    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgba(1.0, 0.0, 0.0, opacity)),
        transform: Transform::from_xyz(pos.x, 0.0, pos.z),
        ..default()
    };
    let entity = commands.spawn(model).id();

    entity
}

pub fn spawn(world: &mut World, pos: Vec3) {
    let mut state: SystemState<Commands> = SystemState::new(world);
    let mut commands = state.get_mut(world);

    commands.spawn((
        TowerMarker,
        TowerPosition {
            x: pos.x as i16,
            z: pos.z as i16,
        },
        super::Marker,
    ));

    state.apply(world);
}

/**
 * Render models for newly-created towers
 */
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    towers: Query<(Entity, &TowerPosition), Without<TowerModel>>,
) {
    for (entity, pos) in towers.iter() {
        let model = spawn_model(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(pos.x as f32, 0.0, pos.z as f32),
            1.0,
        );

        commands.entity(entity).insert(TowerModel(model));
    }
}
