use bevy::{ecs::system::SystemState, prelude::*};

use crate::gui::{
    console,
    utils::{can_place_tower, snap_coords, window_to_world_coords},
};

#[derive(Component)]
pub struct Marker;

#[derive(Bundle)]
struct ModelBundle {
    marker: Marker,
}

#[derive(Resource)]
struct Cursor {
    model: Entity,
}

pub fn handle_draw_cursor(
    world: &mut World,
    maybe_pos: Option<Vec2>,
) -> bool {
    // Init cursor resource
    match world.get_resource::<Cursor>() {
        None => {
            let model =
                crate::towers::basic_tower::spawn_model_with_marker(
                    world,
                    Vec3::new(0.0, 0.0, 0.0),
                    // This should be <1.0, otherwise later opacity changes will have no effect
                    0.0,
                    Marker,
                );

            world.get_resource_or_insert_with(|| Cursor { model });
        }
        Some(_) => {}
    }

    // If position is non-null, spawn a new cursor
    if let Some(cursor_pos) = maybe_pos {
        let pos =
            snap_coords(window_to_world_coords(world, cursor_pos));

        if !can_place_tower(world, pos) {
            return false;
        }

        update_cursor_position(world, pos);
        update_cursor_color(world, 0.5);
    } else {
        update_cursor_color(world, 0.0);
    }

    true
}

fn update_cursor_position(world: &mut World, pos: Vec3) {
    let cursor = world.resource::<Cursor>();
    let mut entity = world.entity_mut(cursor.model);

    let mut transform = entity.get_mut::<Transform>().unwrap();
    transform.translation.x = pos.x;
    transform.translation.z = pos.z;
}

fn update_cursor_color(world: &mut World, opacity: f32) {
    let mut state: SystemState<(
        ResMut<Assets<StandardMaterial>>,
        Query<&Handle<StandardMaterial>, With<Marker>>,
    )> = SystemState::new(world);
    let (mut materials, color_query) = state.get_mut(world);

    let handle = color_query.single();
    let color = materials.get_mut(handle).unwrap();
    color.base_color.set_a(opacity);

    state.apply(world);
}
