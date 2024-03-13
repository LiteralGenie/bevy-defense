use crate::gui::console;
use bevy::prelude::*;

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

#[derive(Copy, Clone)]
pub struct Point2(pub i16, pub i16);

pub struct PathPos {
    pub idx_segment: usize,
    pub pos: Point2,
}

pub struct Path {
    pub id: u8,
    pub points: Vec<PathPos>,
    pub start: Point2,
    pub segments: Vec<Segment>,
}

impl Path {
    pub fn new(
        id: u8,
        start: Point2,
        segments: Vec<Segment>,
    ) -> Path {
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

#[derive(Component)]
pub struct TileModelMarker;
