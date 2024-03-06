use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    gui::utils::{
        can_place_tower, snap_coords, window_to_world_coords,
    },
    towers::basic_tower::spawn_model,
};

#[derive(Resource)]
struct Cursor {
    model: Option<Entity>,
}

pub fn handle_draw_cursor(
    world: &mut World,
    maybe_pos: Option<Vec2>,
) -> bool {
    // Init cursor resource
    world.get_resource_or_insert_with(|| Cursor { model: None });

    // Despawn old cursor
    despawn_cursor(world);

    // If position is non-null, spawn a new cursor
    if let Some(cursor_pos) = maybe_pos {
        let pos =
            snap_coords(window_to_world_coords(world, cursor_pos));

        if !can_place_tower(world, pos) {
            return false;
        }

        let update = spawn_model(world, pos, 127);

        let mut cursor = world
            .get_resource_or_insert_with(|| Cursor { model: None });

        cursor.model = Some(update);
    }

    return true;
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
