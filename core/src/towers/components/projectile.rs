use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub unit: Entity,
    pub model: Entity,
}
