use super::components::InterpolateTranslation;
use bevy::prelude::*;

pub fn interpolate_translation(
    mut query: Query<(Entity, &mut InterpolateTranslation)>,
    mut transform: Query<&mut Transform>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut info) in query.iter_mut() {
        // Check if animation complete
        if info.elapsed >= info.duration {
            commands
                .entity(entity)
                .remove::<InterpolateTranslation>();
            continue;
        } else {
            info.elapsed += 1;
        }

        // Update position
        let elapsed = info.elapsed as f32 + time.overstep_fraction();
        let elapsed_frac = elapsed / info.duration as f32;
        let update = info.pos_start + info.pos_diff * elapsed_frac;

        // Need to handle deleted-model case (eg unit died)
        if let Ok(mut transform) = transform.get_mut(info.model) {
            transform.translation = update;
        }
    }
}
