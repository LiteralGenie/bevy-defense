use bevy::{ecs::system::SystemState, prelude::*};

use crate::gui::{console, utils::window_to_world_coords};

#[derive(Resource)]
struct Cursor {
    model: Option<Entity>,
}

pub fn handle_draw_cursor(
    world: &mut World,
    maybe_pos: Option<Vec2>,
) {
    // Init cursor resource
    world.get_resource_or_insert_with(|| Cursor { model: None });

    // Despawn old cursor
    despawn_cursor(world);

    // If position is non-null, spawn a new cursor
    if let Some(pos) = maybe_pos {
        let update = spawn_model(world, pos);

        let mut cursor = world
            .get_resource_or_insert_with(|| Cursor { model: None });

        cursor.model = Some(update);
    }
}

fn despawn_cursor(world: &mut World) {
    let mut state: SystemState<(Commands, ResMut<Cursor>)> =
        SystemState::new(world);

    let (mut commands, mut cursor) = state.get_mut(world);

    if let Some(model) = cursor.model {
        commands.entity(model).despawn();
        cursor.model = None;
        state.apply(world);
    }
}

fn spawn_model(world: &mut World, pos: Vec2) -> Entity {
    let mut state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
        Query<(&Camera, &GlobalTransform)>,
    )> = SystemState::new(world);

    let (mut commands, mut meshes, mut materials, camera_query) =
        state.get_mut(world);

    let (camera, camera_transform) = camera_query.single();

    let world_pos =
        window_to_world_coords(camera, camera_transform, pos);

    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb_u8(255, 0, 0)),
        transform: Transform::from_xyz(world_pos.x, 0.0, world_pos.z),
        ..default()
    };

    let entity = commands.spawn(model).id();

    state.apply(world);

    entity
}
