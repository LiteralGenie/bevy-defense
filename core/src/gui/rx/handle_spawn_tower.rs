use bevy::prelude::*;

use crate::{
    gui::utils::{snap_coords, window_to_world_coords},
    towers,
};

pub fn handle_spawn_tower(world: &mut World, cursor_pos: Vec2) {
    let pos = snap_coords(window_to_world_coords(world, cursor_pos));
    towers::basic_tower::spawn(world, pos);
}
