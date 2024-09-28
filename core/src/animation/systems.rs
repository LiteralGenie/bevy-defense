use super::components::{
    InterpolateAlpha, InterpolateMaterialColor, InterpolateScale,
    InterpolateTranslation,
};
use crate::gui::console::{self};
use bevy::prelude::*;

pub fn interpolate_translation(
    mut query: Query<(Entity, &mut InterpolateTranslation)>,
    mut transform: Query<&mut Transform>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut info) in query.iter_mut() {
        // Check if animation complete
        if time.is_changed() {
            if info.elapsed >= info.duration {
                commands
                    .entity(entity)
                    .remove::<InterpolateTranslation>();
                continue;
            } else {
                info.elapsed += 1;
            }
        }

        // Update position
        let elapsed_frac = {
            let ticks =
                info.elapsed as f32 + time.overstep_fraction();

            let frac = ticks / info.duration as f32;

            // @jank: The elapsed time can be larger than the specified duration
            //          due to bevy (probably?) deferring the above component removal
            //        So we need to cap this frac to 1.0 to avoid overshooting the end pos
            frac.min(1.0)
        };
        let update = info.pos_start + info.pos_diff * elapsed_frac;

        // Handle deleted-model case (eg unit died)
        if let Ok(mut transform) = transform.get_mut(info.model) {
            transform.translation = update;
        }
    }
}

pub fn interpolate_scale(
    mut query: Query<(Entity, &mut InterpolateScale)>,
    mut transform: Query<&mut Transform>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut info) in query.iter_mut() {
        // Check if animation complete
        if time.is_changed() {
            if info.elapsed >= info.duration {
                commands.entity(entity).remove::<InterpolateScale>();
                continue;
            } else {
                info.elapsed += 1;
            }
        }

        // Update position
        let elapsed_frac = {
            let ticks =
                info.elapsed as f32 + time.overstep_fraction();

            let frac = ticks / info.duration as f32;

            frac.min(1.0)
        };
        let update =
            info.scale_start + info.scale_diff * elapsed_frac;

        // Handle deleted-model case (eg unit died)
        if let Ok(mut transform) = transform.get_mut(info.model) {
            transform.scale = Vec3::splat(update);
        }
    }
}

pub fn interpolate_material_color(
    mut query: Query<(Entity, &mut InterpolateMaterialColor)>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut info) in query.iter_mut() {
        let Some(mat) = materials.get_mut(&info.model) else {
            continue;
        };

        // Check if animation complete
        if time.is_changed() {
            if info.elapsed >= info.duration {
                mat.base_color = Color::srgba(
                    info.end.red,
                    info.end.green,
                    info.end.blue,
                    info.end.alpha,
                );

                commands
                    .entity(entity)
                    .remove::<InterpolateMaterialColor>();

                continue;
            } else {
                info.elapsed += 1;
            }
        }

        // Update
        let elapsed_frac = {
            let ticks =
                info.elapsed as f32 + time.overstep_fraction();

            let frac = ticks / info.duration as f32;

            frac.min(1.0)
        };

        mat.base_color = Color::srgba(
            info.start.red + info.diff.red * elapsed_frac,
            info.start.green + info.diff.green * elapsed_frac,
            info.start.blue + info.diff.blue * elapsed_frac,
            info.start.alpha + info.diff.alpha * elapsed_frac,
        )
    }
}

pub fn interpolate_alpha(
    mut query: Query<(Entity, &mut InterpolateAlpha)>,
    mat_query: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut info) in query.iter_mut() {
        // Check if animation complete
        if time.is_changed() {
            if info.elapsed >= info.duration {
                commands.entity(entity).remove::<InterpolateAlpha>();
                continue;
            } else {
                info.elapsed += 1;
            }
        }

        // Update alpha
        let Ok(handle) = mat_query.get(info.model) else {
            continue;
        };
        let Some(mat) = materials.get_mut(handle) else {
            continue;
        };

        let elapsed_frac = {
            let ticks =
                info.elapsed as f32 + time.overstep_fraction();

            let frac = ticks / info.duration as f32;

            frac.min(1.0)
        };
        let update =
            info.alpha_start + info.alpha_diff * elapsed_frac;

        mat.base_color.set_alpha(update);
    }
}
