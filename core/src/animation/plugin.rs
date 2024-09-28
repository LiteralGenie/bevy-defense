use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                super::systems::tick_despawn_timer,
                super::systems::interpolate_translation,
                super::systems::interpolate_scale,
                super::systems::interpolate_material_color,
                super::systems::interpolate_alpha,
            ),
        );
    }
}
