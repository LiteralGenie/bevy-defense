use std::collections::HashSet;

use super::{BaseDamage, BaseRangeRadius};
use bevy::prelude::*;

#[derive(Component)]
pub struct TowerMarker;

#[derive(Component)]
pub struct TowerModel(pub Entity);

#[derive(Component)]
pub struct TowerPosition {
    pub top_left: (i16, i16),
    pub radius: u8,
    pub coords: HashSet<(i16, i16)>,
}

impl TowerPosition {
    pub fn new(top_left: (i16, i16), radius: u8) -> Self {
        let mut coords = HashSet::from([top_left]);
        let r = radius as i16;
        for x in 1..r {
            for y in 1..r {
                coords.insert((top_left.0 + x, top_left.1 + y));
            }
        }

        Self {
            top_left,
            radius,
            coords,
        }
    }
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
        size_radius: u8,
        range_radius: u8,
    ) -> Self {
        Self {
            marker: TowerMarker,
            base_damage: BaseDamage(damage),
            base_range: BaseRangeRadius(range_radius),
            position: TowerPosition::new(position, size_radius),
            priority: TowerPriority(TowerPriorityTypes::FIRST),
        }
    }
}
