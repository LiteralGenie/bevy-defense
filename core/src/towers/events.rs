use bevy::prelude::*;

#[derive(Event)]
pub struct TowerClickEvent(pub Entity);
