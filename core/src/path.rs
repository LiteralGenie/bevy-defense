use bevy::prelude::*;

use crate::gui::console;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
pub struct Segment {
    pub dir: Direction,
    pub length: u16,
}

#[derive(Copy, Clone)]
pub struct Point2(pub i16, pub i16);

#[derive(Clone)]
pub struct PathPos {
    pub idx_segment: usize,
    pub pos: Point2,
}

#[derive(Component, Clone)]
pub struct Path {
    pub id: u8,
    pub points: Vec<PathPos>,
    pub start: Point2,
    pub segments: Vec<Segment>,
}

impl Path {
    fn new(id: u8, start: Point2, segments: Vec<Segment>) -> Path {
        Path {
            id,
            points: points_from_segments(start, &segments),
            start,
            segments,
        }
    }
}

fn points_from_segments(
    start: Point2,
    segments: &[Segment],
) -> Vec<PathPos> {
    let mut points = vec![PathPos {
        pos: start,
        idx_segment: 0,
    }];

    let mut pos = start;
    for (idx, segment) in segments.iter().enumerate() {
        let step_x: i8 = match segment.dir {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };

        let step_y: i8 = match segment.dir {
            Direction::Down => -1,
            Direction::Up => 1,
            _ => 0,
        };

        for _ in 0..=segment.length {
            pos.0 += step_x as i16;
            pos.1 += step_y as i16;
            points.push(PathPos {
                pos,
                idx_segment: idx,
            });
        }
    }

    points
}

pub fn spawn_paths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let paths = vec![Path::new(
        1,
        Point2(0, 5),
        vec![
            Segment {
                dir: Direction::Down,
                length: 5,
            },
            Segment {
                dir: Direction::Right,
                length: 5,
            },
        ],
    )];

    for path in paths {
        // @fixme: How to write a nested for-loop without littering Clone traits everywhere?
        for pt in path.points.clone() {
            console::log(
                format!("spawning {} {}", pt.pos.0, pt.pos.1)
                    .as_str(),
            );
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::default()),
                material: materials.add(Color::rgb(0.0, 0.5, 0.0)),
                transform: Transform::from_xyz(
                    pt.pos.0 as f32,
                    0.0,
                    pt.pos.1 as f32,
                ),
                ..default()
            });
        }

        commands.spawn(path);
    }
}
