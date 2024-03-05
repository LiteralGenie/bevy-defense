use bevy::prelude::*;
use super::components::*;

#[derive(Bundle)]
pub struct BaseTowerBundle {
    pub pos: TowerPosition,
}
