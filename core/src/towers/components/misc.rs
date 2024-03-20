use std::collections::HashSet;

use crate::gui::console;

use super::{BaseDamage, BaseRangeRadius};
use bevy::prelude::*;

#[derive(Component)]
pub struct TowerMarker;

#[derive(Component)]
pub struct TowerModel(pub Entity);

#[derive(Component)]
pub struct TowerPosition {
    pub top_left: (i16, i16),
    pub size: u8,
    pub coords: HashSet<(i16, i16)>,
}

impl TowerPosition {
    pub fn new(top_left: (i16, i16), size: u8) -> Self {
        let mut coords = HashSet::new();
        let sz = size as i16;
        for x in 0..sz {
            for y in 0..sz {
                coords.insert((top_left.0 + x, top_left.1 - y));
            }
        }

        Self {
            top_left,
            size,
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
        size: u8,
        range_radius: u8,
    ) -> Self {
        Self {
            marker: TowerMarker,
            base_damage: BaseDamage(damage),
            base_range: BaseRangeRadius(range_radius),
            position: TowerPosition::new(position, size),
            priority: TowerPriority(TowerPriorityTypes::FIRST),
        }
    }
}
