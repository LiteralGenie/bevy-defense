use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModel(pub Entity);

#[derive(Component)]
pub struct UnitHealthBarModel(pub Entity);
