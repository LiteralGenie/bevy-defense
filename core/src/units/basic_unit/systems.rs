use super::super::components::UnitModel;
use super::super::health_bar::build_health_bar;
use crate::components::DoNotRender;
use crate::gui::console;
use bevy::prelude::*;

// @todo: consider moving the model generation into a utility function
//        and refactoring the common component spawning / health bar spawning logic
//        into a single system instead of having a per-unit render system like this
//        (like how towers work)
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    units: Query<
        Entity,
        (
            With<super::Marker>,
            Without<UnitModel>,
            Without<DoNotRender>,
        ),
    >,
) {
    for entity in units.iter() {
        let health_bar_model_id = commands
            .spawn(build_health_bar(&mut meshes, &mut materials))
            .id();

        let mut model = commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });

        model.add_child(health_bar_model_id);

        let model_id = model.id();

        commands.entity(entity).insert(UnitModel(model_id));
    }
}
