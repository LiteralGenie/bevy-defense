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
}

impl BaseTowerBundle {
    pub fn new(position: (i16, i16)) -> Self {
        Self {
            marker: TowerMarker,
            position: TowerPosition {
                x: position.0,
                z: position.1,
            },
            priority: TowerPriority(TowerPriorityTypes::FIRST),
        }
    }
}
