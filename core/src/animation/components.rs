use bevy::prelude::*;

#[derive(Component)]
pub struct InterpolateTranslation {
    pub model: Entity,
    pub pos_start: Vec3,
    pub pos_end: Vec3,
    pub pos_diff: Vec3,
    pub duration: u32,
    pub elapsed: u32,
    pub rem_delay: u16,
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
            rem_delay: 0,
            pos_start,
            pos_end,
            pos_diff: Vec3::new(
                pos_end.x - pos_start.x,
                pos_end.y - pos_start.y,
                pos_end.z - pos_start.z,
            ),
        }
    }

    pub fn delay(mut self, t: u16) -> Self {
        self.rem_delay = t;
        return self;
    }
}

#[derive(Component)]
pub struct InterpolateScale {
    pub model: Entity,
    pub scale_start: f32,
    pub scale_end: f32,
    pub scale_diff: f32,
    pub duration: u32,
    pub elapsed: u32,
    pub rem_delay: u16,
}

impl InterpolateScale {
    pub fn new(
        model: Entity,
        duration: u32,
        scale_start: f32,
        scale_end: f32,
    ) -> Self {
        Self {
            model,
            duration,
            elapsed: 0,
            rem_delay: 0,
            scale_start,
            scale_end,
            scale_diff: scale_end - scale_start,
        }
    }

    pub fn delay(mut self, t: u16) -> Self {
        self.rem_delay = t;
        return self;
    }
}

#[derive(Component)]
pub struct InterpolateMaterialColor {
    pub model: Handle<StandardMaterial>,
    pub start: Srgba,
    pub end: Srgba,
    pub diff: Srgba,
    pub duration: u32,
    pub elapsed: u32,
    pub rem_delay: u16,
}

impl InterpolateMaterialColor {
    pub fn new(
        model: Handle<StandardMaterial>,
        duration: u32,
        start: Srgba,
        end: Srgba,
    ) -> Self {
        let r = end.red - start.red;
        let g = end.green - start.green;
        let b = end.blue - start.blue;
        let a = end.alpha - start.alpha;
        let srgba_diff = Srgba::new(r, g, b, a);

        Self {
            model,
            duration,
            elapsed: 0,
            rem_delay: 0,
            start,
            end,
            diff: srgba_diff,
        }
    }

    pub fn delay(mut self, t: u16) -> Self {
        self.rem_delay = t;
        return self;
    }
}

#[derive(Component)]
pub struct InterpolateAlpha {
    pub model: Entity,
    pub alpha_start: f32,
    pub alpha_end: f32,
    pub alpha_diff: f32,
    pub duration: u32,
    pub elapsed: u32,
    pub rem_delay: u16,
}

impl InterpolateAlpha {
    pub fn new(
        model: Entity,
        duration: u32,
        alpha_start: f32,
        alpha_end: f32,
    ) -> Self {
        Self {
            model,
            duration,
            elapsed: 0,
            rem_delay: 0,
            alpha_start,
            alpha_end,
            alpha_diff: alpha_end - alpha_start,
        }
    }

    pub fn delay(mut self, t: u16) -> Self {
        self.rem_delay = t;
        return self;
    }
}

#[derive(Component)]
pub struct DespawnTimer {
    pub entity: Entity,
    pub rem_delay: u16,
}

impl DespawnTimer {
    pub fn new(entity: Entity, delay: u16) -> Self {
        Self {
            entity,
            rem_delay: delay,
        }
    }
}
