use super::components::InterpolateTranslation;
use crate::timers::tick_timer::TickTimer;
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
        let elapsed = tick.0 as f32 + time.overstep_fraction();
        let elapsed_frac = elapsed / info.duration as f32;
        let update = info.pos_start + info.pos_diff * elapsed_frac;

        let mut transform = transform.get_mut(info.model).unwrap();
        transform.translation = update;
    }
}
