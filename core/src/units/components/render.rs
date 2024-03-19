use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModel(pub Entity);

#[derive(Component)]
pub struct UnitHealthBarModel(pub Entity);

#[derive(Component)]
pub struct UnitMovementInfo {
    pub start: Vec3,
    pub end: Vec3,
    pub diff: Vec3,
}

impl UnitMovementInfo {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self {
            start,
            end,
            diff: Vec3::new(
                end.x - start.x,
                end.y - start.y,
                end.z - start.z,
            ),
        }
    }

    pub fn interpolate(&self, frac: f32) -> Vec3 {
        let result = self.start + frac * (self.end - self.start);
        result
    }
}
