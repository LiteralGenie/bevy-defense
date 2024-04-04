use bevy::prelude::*;

use crate::{
    gui::utils::{snap_coords, window_to_world_coords},
    towers::{components::spawn_tower_bundle, config::match_config},
};

pub fn handle_spawn_tower(
    world: &mut World,
    id_tower: u16,
    cursor_pos: (f32, f32),
) {
    let cfg = match_config(id_tower);
    let pos = snap_coords(window_to_world_coords(world, cursor_pos));
    spawn_tower_bundle(world, &cfg, pos);
}
