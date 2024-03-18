use super::{BaseDamage, BaseRangeRadius};
use bevy::prelude::*;

#[derive(Component)]
pub struct TowerMarker;

#[derive(Component)]
pub struct TowerModel(pub Entity);

#[derive(Component)]
pub struct TowerPosition {
    pub x: i16,
    pub z: i16,
}

pub enum TowerPriorityTypes {
    FIRST,
    LAST,
    WEAKEST,
    STRONGEST,
}

#[derive(Component)]
pub struct TowerPriority(pub TowerPriorityTypes);

#[derive(Bundle)]
pub struct BaseTowerBundle {
    marker: TowerMarker,

    position: TowerPosition,
    priority: TowerPriority,

    base_damage: BaseDamage,
    base_range: BaseRangeRadius,
}

impl BaseTowerBundle {
    pub fn new(
        position: (i16, i16),
        damage: u32,
        radius: u8,
    ) -> Self {
        Self {
            marker: TowerMarker,
            base_damage: BaseDamage(damage),
            base_range: BaseRangeRadius(radius),
            position: TowerPosition {
                x: position.0,
                z: position.1,
            },
            priority: TowerPriority(TowerPriorityTypes::FIRST),
        }
    }
}
