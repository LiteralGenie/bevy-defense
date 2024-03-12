use bevy::prelude::*;

use crate::{
    gui::console,
    units::components::{
        UnitDist, UnitHealth, UnitMarker, UnitModel, UnitPathId,
        UnitStatus, UnitStatusTypes,
    },
};

pub fn spawn(mut commands: &mut Commands, path_id: u8) {
    commands.spawn((
        UnitMarker,
        UnitStatus(UnitStatusTypes::PRESPAWN),
        UnitDist(0),
        UnitHealth(100),
        UnitPathId(path_id),
        super::Marker,
    ));
}

pub fn render_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    units: Query<
        (Entity, &UnitStatus, &UnitDist),
        (With<super::Marker>, Without<UnitModel>),
    >,
) {
    for (entity, status, dist) in units.iter() {
        let is_alive = matches!(status.0, UnitStatusTypes::ALIVE);
        let opacity = if is_alive { 1.0 } else { 0.0 };
        console::log(
            format!(
                "Rendering unit at {} with opacity {}",
                dist.0, opacity
            )
            .as_str(),
        );

        let model = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::default()),
                material: materials
                    .add(Color::rgba(0.0, 0.5, 0.0, opacity)),
                transform: Transform::from_xyz(
                    dist.0 as f32,
                    0.5,
                    dist.0 as f32,
                ),
                ..default()
            })
            .id();

        commands.entity(entity).insert(UnitModel(model));
    }
}

pub fn render_movement(
    units: Query<
        (&UnitDist, &UnitModel),
        (With<super::Marker>, With<UnitModel>, Changed<UnitDist>),
    >,
    mut models: Query<&Transform>,
) {
    for (dist, model) in units.iter() {
        let transform = models.get_mut(model.0).unwrap();
        let mut translation = transform.translation;
        translation.x = dist.0 as f32;
        translation.y = dist.0 as f32;
    }
}
