use super::components::{UnitHealth, UnitHealthMax, UnitModel};
use crate::gui::console;
use bevy::prelude::*;

pub fn build_health_bar(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    let model = PbrBundle {
        mesh: meshes.add(Rectangle::new(0.75, 0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 6.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    };

    return model;
}

// Scale health bar model by unit health
pub fn render_health_bar(
    unit_query: Query<
        (&UnitHealth, &UnitHealthMax, &UnitModel),
        Changed<UnitHealth>,
    >,
    mut transform_query: Query<&mut Transform>,
) {
    for (health, health_max, model) in unit_query.iter() {
        let mut transform =
            transform_query.get_mut(model.health_bar).unwrap();

        let health_percent = health.0 as f32 / health_max.0 as f32;

        transform.scale = Vec3::new(health_percent, 1.0, 1.0);

        // The scale shrinks left / right sides of health bar by X
        // so to keep it left aligned, translate it to the left by X / 2
        transform.translation.x = -(1.0 - health_percent) / 2.0;

        break;
    }
}
