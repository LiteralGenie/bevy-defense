use bevy::{ecs::system::SystemState, prelude::*};

use crate::gui::{
    console,
    utils::{can_place_tower, snap_coords, window_to_world_coords},
};

#[derive(Resource)]
struct Cursor {
    model: Entity,
}

/**
 * Draw a ghost thing underneath the cursor
 */
pub fn handle_draw_cursor(
    world: &mut World,
    maybe_pos: Option<Vec2>,
) -> bool {
    // Init cursor resource
    match world.get_resource::<Cursor>() {
        None => init_resource(world),
        Some(_) => {}
    }

    // If cursor pos is non-null, update position of cursor ghost
    if let Some(cursor_pos) = maybe_pos {
        let pos =
            snap_coords(window_to_world_coords(world, cursor_pos));

        if !can_place_tower(world, pos) {
            return false;
        }

        update_cursor_position(world, (pos.0 as f32, pos.1 as f32));
        update_cursor_color(world, 0.5);
    } else {
        // Otherwise hide the ghost
        update_cursor_color(world, 0.0);
    }

    true
}

fn init_resource(world: &mut World) {
    let mut state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )> = SystemState::new(world);

    let (mut commands, mut meshes, mut materials) =
        state.get_mut(world);

    let model = crate::towers::basic_tower::spawn_model(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
    );

    world.insert_resource(Cursor { model });

    state.apply(world);
}

fn update_cursor_position(world: &mut World, pos: (f32, f32)) {
    let cursor = world.resource::<Cursor>();
    let mut entity = world.entity_mut(cursor.model);

    let mut transform = entity.get_mut::<Transform>().unwrap();
    transform.translation.x = pos.0;
    transform.translation.z = pos.1;
}

fn update_cursor_color(world: &mut World, opacity: f32) {
    let mut state: SystemState<(
        ResMut<Assets<StandardMaterial>>,
        Res<Cursor>,
        Query<&Handle<StandardMaterial>>,
    )> = SystemState::new(world);

    let (mut materials, cursor, mut color_query) =
        state.get_mut(world);

    let handle = color_query.get_mut(cursor.model).unwrap();
    let mat = materials.get_mut(handle).unwrap();
    mat.alpha_mode = AlphaMode::Blend;
    mat.base_color.set_a(opacity);

    state.apply(world);
}
