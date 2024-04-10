use crate::{gui::console, towers::config::TowerConfig};
use bevy::prelude::*;
use std::collections::HashSet;

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
