use bevy::prelude::*;
use crate::towers::components::*;

#[derive(Component)]
pub struct Marker;

#[derive(Bundle)]
pub struct Bundle {
    marker: Marker,
    pub pos: TowerPosition,
}
