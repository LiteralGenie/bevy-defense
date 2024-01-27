use bevy::prelude::*;
use super::components::*;

#[derive(Bundle)]
pub struct BasicUnitBundle {
    pub dist: UnitDist,
    pub hp: UnitHealth,
}
