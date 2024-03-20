use crate::gui::console;
use bevy::{prelude::*, utils::HashSet};

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Segment {
    pub dir: Direction,
    pub length: u16,
}

type Point2 = (i16, i16);

pub struct PathPos {
    pub idx_segment: usize,
    pub pos: Point2,
}

pub struct Path {
    pub id: u8,
    pub points: Vec<PathPos>,
    pub buffer_points: HashSet<Point2>,
    pub start: Point2,
    pub segments: Vec<Segment>,
}

impl Path {
    pub fn new(
        id: u8,
        start: Point2,
        segments: Vec<Segment>,
    ) -> Path {
        let (points, buffer_points) =
            points_from_segments(start, &segments);

        Path {
            id,
            points,
            buffer_points,
            start,
            segments,
        }
    }
}

fn points_from_segments(
    start: Point2,
    segments: &[Segment],
) -> (Vec<PathPos>, HashSet<Point2>) {
    let mut points = vec![PathPos {
        pos: start,
        idx_segment: 0,
    }];

    let mut buffer_points = HashSet::<Point2>::new();
    match segments.get(0).unwrap().dir {
        Direction::Left => {
            buffer_points.insert((start.0, start.1 - 1));
            buffer_points.insert((start.0, start.1 + 1));
        }
        Direction::Right => {
            buffer_points.insert((start.0, start.1 - 1));
            buffer_points.insert((start.0, start.1 + 1));
        }
        Direction::Down => {
            buffer_points.insert((start.0 - 1, start.1));
            buffer_points.insert((start.0 + 1, start.1));
        }
        Direction::Up => {
            buffer_points.insert((start.0 - 1, start.1));
            buffer_points.insert((start.0 + 1, start.1));
        }
    }

    let mut pos = start;
    for (idx_segment, segment) in segments.iter().enumerate() {
        let (step_x, step_y): (i16, i16) = match segment.dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Up => (1, 0),
        };

        let (buffer_x, buffer_y): (i16, i16) = match segment.dir {
            Direction::Left => (0, 1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Up => (1, 0),
        };

        for idx_pt in 0..=segment.length {
            pos.0 += step_x as i16;
            pos.1 += step_y as i16;

            points.push(PathPos { pos, idx_segment });

            buffer_points
                .insert((pos.0 - buffer_x, pos.1 - buffer_y));
            buffer_points
                .insert((pos.0 + buffer_x, pos.1 + buffer_y));

            if idx_pt == segment.length {
                let next_pt = (pos.0 + step_x, pos.1 + step_y);

                buffer_points.insert((
                    next_pt.0 - buffer_x,
                    next_pt.1 - buffer_y,
                ));
                buffer_points.insert(next_pt);
                buffer_points.insert((
                    next_pt.0 + buffer_x,
                    next_pt.1 + buffer_y,
                ));
            }
        }
    }

    (points, buffer_points)
}

#[derive(Component)]
pub struct TileModelMarker;
