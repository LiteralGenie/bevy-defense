use bevy::prelude::*;

// Marks units / towers / etc that have been intentionally despawned
#[derive(Component)]
pub struct DoNotRender;
