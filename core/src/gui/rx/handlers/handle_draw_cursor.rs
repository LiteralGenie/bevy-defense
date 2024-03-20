use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    gui::{
        console,
        utils::{
            can_place_tower, snap_coords, window_to_world_coords,
        },
    },
    towers::{
        components::BasicRangeType,
        matchers::{match_range_radius, match_size},
    },
};

#[derive(Resource)]
struct Cursor {
    model: Entity,
    range_model: Entity,
}

/**
 * Draw a ghost thing underneath the cursor
 */
pub fn handle_draw_cursor(
    world: &mut World,
    data: Option<((f32, f32), u16)>,
) -> bool {
    // Init cursor resource
    match world.get_resource::<Cursor>() {
        None => init_resource(world),
        Some(_) => {}
    }

    // If cursor pos is non-null, update position of cursor ghost
    if let Some((cursor_pos, id_tower)) = data {
        let pos =
            snap_coords(window_to_world_coords(world, cursor_pos));

        if !can_place_tower(world, pos, id_tower) {
            return false;
        }

        update_cursor_visbility(world, true);
        update_cursor_position(world, (pos.0 as f32, pos.1 as f32));
    } else {
        // Otherwise hide the ghost
        update_cursor_visbility(world, false);
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

    let id = 0;
    let range_model = commands.spawn(SpatialBundle::default()).id();
    let points = BasicRangeType::compute_points(
        match_range_radius(id),
        (0, 0),
        match_size(id),
    );
    for pt in points {
        let tile = super::render_tile_highlight(
            &pt,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        commands.entity(range_model).push_children(&[tile]);
    }

    world.insert_resource(Cursor { model, range_model });
    state.apply(world);

    update_cursor_color(world, 0.5);
}

fn update_cursor_position(world: &mut World, pos: (f32, f32)) {
    let cursor = world.resource::<Cursor>();
    let mut entity = world.entity_mut(cursor.model);
    let mut transform = entity.get_mut::<Transform>().unwrap();
    transform.translation.x = pos.0;
    transform.translation.z = pos.1;

    let cursor = world.resource::<Cursor>();
    let mut entity = world.entity_mut(cursor.range_model);
    let mut transform = entity.get_mut::<Transform>().unwrap();
    transform.translation.x = pos.0;
    transform.translation.z = pos.1;
}

fn update_cursor_color(world: &mut World, opacity: f32) {
    let mut state: SystemState<(
        Res<Cursor>,
        ResMut<Assets<StandardMaterial>>,
        Query<&Handle<StandardMaterial>>,
        Query<&Children>,
    )> = SystemState::new(world);

    let (cursor, mut materials, mut color_query, children_query) =
        state.get_mut(world);

    let model =
        children_query.get(cursor.model).unwrap().first().unwrap();
    let handle = color_query.get_mut(*model).unwrap();
    let mat = materials.get_mut(handle).unwrap();
    mat.alpha_mode = AlphaMode::Blend;
    mat.base_color.set_a(opacity);

    state.apply(world);
}

fn update_cursor_visbility(world: &mut World, is_visible: bool) {
    let mut state: SystemState<(
        Res<Cursor>,
        Query<&mut Visibility>,
    )> = SystemState::new(world);

    let (cursor, mut query) = state.get_mut(world);

    let update = if is_visible {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    let mut visibility = query.get_mut(cursor.model).unwrap();
    *visibility = update;

    let mut visibility = query.get_mut(cursor.range_model).unwrap();
    *visibility = update;

    state.apply(world);
}
