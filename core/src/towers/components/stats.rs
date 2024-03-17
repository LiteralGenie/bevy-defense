use bevy::prelude::*;

#[derive(Component)]
pub struct BaseDamage(pub u32);

#[derive(Component)]
pub struct EffectiveDamage(pub u32);

#[derive(Component)]
pub struct BaseRangeRadius(pub u8);

#[derive(Component)]
pub struct EffectiveRangeRadius(pub u8);
