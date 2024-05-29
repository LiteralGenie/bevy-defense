use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModel {
    pub root: Entity,
    pub base: Entity,
    pub health_bar: Entity,
}

#[derive(Component)]
pub struct UnitHealthBarModel(pub Entity);
