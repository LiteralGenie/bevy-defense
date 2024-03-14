use super::components::{UnitHealth, UnitModel};
use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBarMarker;

pub fn build_health_bar(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (PbrBundle, HealthBarMarker) {
    let model = PbrBundle {
        mesh: meshes.add(Rectangle::new(0.75, 0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    };

    return (model, HealthBarMarker);
}

// Scale health bar model by unit health
pub fn render_health_bar(
    unit_query: Query<(&UnitHealth, &UnitModel), Changed<UnitHealth>>,
    children_query: Query<&Children>,
    mut model_query: Query<(&HealthBarMarker, &mut Transform)>,
) {
    for (health, model) in unit_query.iter() {
        let children = children_query.get(model.0).unwrap();

        for child in children.iter() {
            if let Ok(res) = model_query.get_mut(*child) {
                let (_, mut transform) = res;

                let health_percent = health.0 as f32 / 100.0;

                transform.scale = Vec3::new(health_percent, 1.0, 1.0);

                transform.translation.x =
                    -(1.0 - health_percent) / 2.0;

                break;
            }
        }
    }
}
