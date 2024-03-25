use bevy::prelude::*;

#[derive(Component)]
pub struct InterpolateTranslation {
    pub model: Entity,
    pub pos_start: Vec3,
    pub pos_end: Vec3,
    pub pos_diff: Vec3,
    pub duration: u32,
    pub elapsed: u32,
}

impl InterpolateTranslation {
    pub fn new(
        model: Entity,
        duration: u32,
        pos_start: Vec3,
        pos_end: Vec3,
    ) -> Self {
        Self {
            model,
            duration,
            elapsed: 0,
            pos_start,
            pos_end,
            pos_diff: Vec3::new(
                pos_end.x - pos_start.x,
                pos_end.y - pos_start.y,
                pos_end.z - pos_start.z,
            ),
        }
    }
}
