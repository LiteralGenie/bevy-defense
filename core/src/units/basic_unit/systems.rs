use crate::units::components::UnitModel;
use bevy::prelude::*;

pub fn render_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    units: Query<Entity, (With<super::Marker>, Without<UnitModel>)>,
) {
    for entity in units.iter() {
        let model = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::default()),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.0, 0.0, 0.5),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            })
            .id();

        commands.entity(entity).insert(UnitModel(model));
    }
}
