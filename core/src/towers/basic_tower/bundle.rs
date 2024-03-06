use crate::towers::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

#[derive(Bundle)]
pub struct Bundle {
    base_marker: TowerMarker,
    type_marker: Marker,
    pub pos: TowerPosition,
}
