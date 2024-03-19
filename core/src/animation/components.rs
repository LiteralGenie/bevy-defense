use bevy::prelude::*;

#[derive(Component)]
pub struct InterpolateTranslation {
    pub model: Entity,
    pub pos_start: Vec3,
    pub pos_end: Vec3,
    pub pos_diff: Vec3,
    pub tick_start: u32,
    pub tick_end: u32,
    pub duration: u32,
}

impl InterpolateTranslation {
    pub fn new(
        model: Entity,
        pos_start: Vec3,
        pos_end: Vec3,
        tick_start: u32,
        tick_end: u32,
    ) -> Self {
        Self {
            model,
            pos_start,
            pos_end,
            pos_diff: Vec3::new(
                pos_end.x - pos_start.x,
                pos_end.y - pos_start.y,
                pos_end.z - pos_start.z,
            ),
            tick_start,
            tick_end,
            duration: tick_end - tick_start,
        }
    }
}
