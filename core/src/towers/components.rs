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

#[derive(Bundle)]
pub struct BaseTowerBundle {
    marker: TowerMarker,
    position: TowerPosition,
}

impl BaseTowerBundle {
    pub fn new(position: (i16, i16)) -> Self {
        Self {
            marker: TowerMarker,
            position: TowerPosition {
                x: position.0,
                z: position.1,
            },
        }
    }
}
