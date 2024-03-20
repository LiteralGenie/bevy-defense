use super::{path, Point2, Scenario, Wave, WaveEnemy};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn spawn_scenario(mut commands: Commands) {
    let paths = HashMap::from([(
        1,
        path::Path::new(
            1,
            path::Point2(5, 10),
            vec![
                path::Segment {
                    dir: path::Direction::Down,
                    length: 5,
                },
                path::Segment {
                    dir: path::Direction::Left,
                    length: 5,
                },
                path::Segment {
                    dir: path::Direction::Down,
                    length: 5,
                },
                path::Segment {
                    dir: path::Direction::Right,
                    length: 5,
                },
                path::Segment {
                    dir: path::Direction::Down,
                    length: 10,
                },
            ],
        ),
    )]);

    let mut scenario = Scenario {
        paths,
        waves: Vec::from([Wave {
            enemies: Vec::new(),
        }]),
    };

    for i in 1..99 {
        scenario.waves[0].enemies.push(WaveEnemy {
            id_unit: 0,
            id_path: 1,
            delay: i * 3,
        });
    }

    for i in 1..10 {
        scenario.waves[0].enemies.push(WaveEnemy {
            id_unit: 1,
            id_path: 1,
            delay: i * 20,
        });
    }

    commands.insert_resource(scenario);
}

pub fn render_paths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scenario: Res<Scenario>,
) {
    let mut points = HashSet::<Point2>::new();

    for path in scenario.paths.values() {
        for pt in path.points.iter() {
            points.insert(pt.pos);
        }

        for pt in path.buffer_points.iter() {
            points.insert(*pt);
        }
    }

    for pt in points {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::default()),
                material: materials.add(Color::rgb(0.0, 0.5, 0.0)),
                transform: Transform::from_xyz(
                    pt.0 as f32,
                    0.0,
                    pt.1 as f32,
                ),
                ..default()
            },
            super::TileModelMarker,
        ));
    }
}
