use super::components::InterpolateTranslation;
use crate::{gui::console, timers::tick_timer::TickTimer};
use bevy::prelude::*;

pub fn interpolate_translation(
    query: Query<(Entity, &InterpolateTranslation)>,
    mut transform: Query<&mut Transform>,
    time: Res<Time<Fixed>>,
    tick: Res<TickTimer>,
    mut commands: Commands,
) {
    for (entity, info) in query.iter() {
        // Check if animation complete
        if tick.0 >= info.tick_end {
            commands
                .entity(entity)
                .remove::<InterpolateTranslation>();
            continue;
        }

        // Update position
        let elapsed = (tick.0 - info.tick_start) as f32
            + time.overstep_fraction();
        let elapsed_frac = elapsed / info.duration as f32;
        let update = info.pos_start + info.pos_diff * elapsed_frac;

        // Need to handle deleted-model case (eg unit died)
        if let Ok(mut transform) = transform.get_mut(info.model) {
            transform.translation = update;
        }
    }
}
