use super::super::components::{TowerModel, TowerPosition};
use super::spawn_model;
use bevy::prelude::*;

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
        );

        commands.entity(entity).insert(TowerModel(model));
    }
}
