use std::collections::HashSet;

use crate::{gui::console, towers::config::match_config};

use super::{BaseAttackSpeed, BaseDamage, BaseRangeRadius};
use bevy::prelude::*;

#[derive(Component)]
pub struct TowerMarker(pub u16);

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

#[derive(Component)]
pub struct TowerAttackEnergy {
    pub acc: u8,
    pub charges: u8,
}

#[derive(Bundle)]
pub struct BaseTowerBundle {
    marker: TowerMarker,

    energy: TowerAttackEnergy,
    position: TowerPosition,
    priority: TowerPriority,

    base_damage: BaseDamage,
    base_range: BaseRangeRadius,
    base_speed: BaseAttackSpeed,
}

impl BaseTowerBundle {
    pub fn new(id_tower: u16, position: (i16, i16)) -> Self {
        let cfg = match_config(id_tower);

        Self {
            marker: TowerMarker(id_tower),
            energy: TowerAttackEnergy { acc: 0, charges: 0 },
            position: TowerPosition::new(position, cfg.size),
            priority: TowerPriority(TowerPriorityTypes::FIRST),
            base_damage: BaseDamage(cfg.damage),
            base_range: BaseRangeRadius(cfg.range_radius),
            base_speed: BaseAttackSpeed(cfg.speed),
        }
    }
}
