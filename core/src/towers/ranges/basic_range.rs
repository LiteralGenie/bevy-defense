use crate::scenario::Scenario;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Component)]
pub struct BasicRange {
    radius: u8,
    points: HashSet<(i16, i16)>,
    // Map where key is a path id, and values are the dists where this range intersect
    path_intersections: HashMap<u8, HashSet<u16>>,
}

impl BasicRange {
    pub fn new(
        radius: u8,
        center: (i16, i16),
        scenario: Res<Scenario>,
    ) -> Self {
        let r = radius as i16;

        let mut points = HashSet::new();

        // Draw one quarter of the range
        // eg, for r=2, each iteration of the inner y-loop adds...
        // ----- ----- --x-- --x-- --x--- --x--
        // ----- --x-- --x-- --x-- --xx-- --xx-
        // --x-- --x-- --x-- --xx- --xx-- --xxx
        // ----- ----- ----- ----- ------ -----
        // ----- ----- ----- ----- ------ -----
        for x in 0..(r + 1) {
            let height = r - x;
            for y in 0..(height + 1) {
                points.insert((center.0 + x, center.0 + y));
            }
        }

        // Flip the quarter horizontally, making a half
        // --x--       --x--
        // --xx-       -xxx-
        // --xxx  ...  xxxxx
        // -----       -----
        // -----       -----
        let mut added = HashSet::new();
        for pt in points.iter() {
            added.insert((center.0 - pt.0, pt.1));
        }
        points.extend(added);

        // Flip the half vertically, completing the range
        // --x--       --x--
        // -xxx-       -xxx-
        // xxxxx  ...  xxxxx
        // -----       -xxx-
        // -----       --x--
        let mut added = HashSet::new();
        for pt in points.iter() {
            added.insert((pt.0, -pt.1));
        }
        points.extend(added);

        // Find points that lie on a path and cache them
        // (as distances from the start of path)
        let mut path_intersections = HashMap::new();
        for path in scenario.paths.values() {
            let mut bin = HashSet::new();

            for (idx, pt) in path.points.iter().enumerate() {
                let pt = (pt.pos.0, pt.pos.1);
                if points.contains(&pt) {
                    bin.insert(idx as u16);
                }
            }

            path_intersections.insert(path.id, bin);
        }

        Self {
            radius,
            points,
            path_intersections,
        }
    }
}
