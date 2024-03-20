use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    gui::console,
    towers::components::{TowerPosition, TowerRange},
};

#[derive(Resource)]
struct RangeHighlight {
    models: Vec<Entity>,
}

/**
 * Highlight tiles in range of tower
 */
pub fn handle_draw_range(world: &mut World, id_tower: Option<u64>) {
    // Delete old range
    despawn_highlight(world);

    // Draw range for target tower
    if let Some(id) = id_tower {
        let entity = world.entity(Entity::from_bits(id)).id();

        let mut state: SystemState<(
            Query<&TowerRange>,
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
        )> = SystemState::new(world);

        let (query, mut commands, mut meshes, mut materials) =
            state.get_mut(world);

        let range = query.get(entity).unwrap();

        let mut models = Vec::new();
        for pt in range.points.iter() {
            models.push(render_tile_highlight(
                pt,
                &mut commands,
                &mut meshes,
                &mut materials,
            ));
        }

        world.insert_resource(RangeHighlight { models });

        state.apply(world)
    }
}

pub fn render_tile_highlight(
    pos: &(i16, i16),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 0.1, 1.0)),
            material: materials.add(Color::rgba(0.5, 0.0, 0.5, 0.5)),
            transform: Transform::from_xyz(
                pos.0 as f32,
                0.0,
                pos.1 as f32,
            ),
            ..default()
        })
        .id()
}

pub fn despawn_highlight(world: &mut World) {
    let mut state: SystemState<(
        Option<ResMut<RangeHighlight>>,
        Commands,
    )> = SystemState::new(world);

    let (highlight, mut commands) = state.get_mut(world);

    if let Some(mut highlight) = highlight {
        for entity in highlight.models.iter() {
            commands.entity(*entity).despawn();
        }

        highlight.models.clear();

        state.apply(world);
    }
}
