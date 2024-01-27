use bevy::prelude::*;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Segment {
    pub dir: Direction,
    pub length: u16,
}

#[derive(Copy, Clone)]
struct Point2(pub u16, pub u16);

struct PathPos {
    idx_segment: usize,
    pos: Point2,
}

#[derive(Resource)]
pub struct Path {
    id: u8,
    points: Vec<PathPos>,
    start: Point2,
    segments: Vec<Segment>,
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

fn points_from_segments(start: Point2, segments: &[Segment]) -> Vec<PathPos> {
    let mut points = vec![PathPos { pos: start, idx_segment: 0 }];

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
            pos.0 = pos.0.wrapping_add(step_x as u16);
            pos.1 = pos.1.wrapping_add(step_y as u16);
            points.push(PathPos { pos, idx_segment: idx });
        }
    }

    return points;
}

pub fn load_paths() {
    let paths = vec![
        Path::new(
            1,
            Point2(10, 10),
            vec![
                Segment {
                    dir: Direction::Down,
                    length: 5,
                },
                Segment {
                    dir: Direction::Right,
                    length: 5,
                }
            ]
        )
    ];
}
