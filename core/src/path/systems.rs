use bevy::prelude::*;

pub fn spawn_paths(mut commands: Commands) {
    let paths = vec![super::Path::new(
        1,
        super::Point2(0, 5),
        vec![
            super::Segment {
                dir: super::Direction::Down,
                length: 5,
            },
            super::Segment {
                dir: super::Direction::Right,
                length: 5,
            },
        ],
    )];

    for path in paths {
        commands.spawn(path);
    }
}

pub fn render_paths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    paths: Query<&super::Path, Added<super::Path>>,
) {
    for path in paths.iter() {
        for pt in path.points.iter() {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::default()),
                    material: materials
                        .add(Color::rgb(0.0, 0.5, 0.0)),
                    transform: Transform::from_xyz(
                        pt.pos.0 as f32,
                        0.0,
                        pt.pos.1 as f32,
                    ),
                    ..default()
                },
                super::TileModelMarker,
            ));
        }
    }
}
