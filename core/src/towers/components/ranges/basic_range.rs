use super::TowerRange;
use crate::scenario::Scenario;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Component)]
pub struct BasicRangeType;

impl BasicRangeType {
    pub fn create(
        radius: u8,
        top_left: (i16, i16),
        axis_width: u8,
        scenario: &Res<Scenario>,
    ) -> TowerRange {
        let r = radius as i16;
        let w = axis_width as i16;

        let mut points = HashSet::new();

        // Draw one quarter of the range, with the center square's top-left corner at (0,0)
        // eg, for r=3, each iteration of the inner y-loop adds...
        // ---|--- ---|--- ---|---
        // ---|--- --x|--- --x|---
        // --x|--- --x|--- -xx|---
        // ===o=== ===o=== ===o===
        // ---|--- ---|--- ---|---
        // ---|--- ---|--- ---|---
        // ---|--- ---|--- ---|---
        for x in 1..r {
            let height = r - x;
            for y in 1..=height {
                points.insert((-x, y));
            }
        }

        // Mirror across y-axis, accounting for axis width
        // eg, for w=2...
        // ----|----       ----|----
        // ---x|----       ---x|-x--
        // --xx|----       --xx|-xx-
        // ====o====  ...  ====o====
        // ----|----       ----|----
        // ----|----       ----|----
        // ----|----       ----|----
        let mut added = HashSet::new();
        for pt in points.iter() {
            added.insert((-pt.0 + (w - 1), pt.1));
        }
        points.extend(added);

        // Mirror across x-axis, still accounting for axis width
        // ---|-----       ---|-----
        // --x|-x---       --x|-x---
        // -xx|-xx--       -xx|-xx--
        // ===o=====  ...  ===o=====
        // ---|-----       ---|-----
        // ---|-----       -xx|--xx-
        // ---|-----       --x|--x--
        // ---|-----       ---|-----
        let mut added = HashSet::new();
        for pt in points.iter() {
            added.insert((pt.0, -pt.1 - (w - 1)));
        }
        points.extend(added);

        // Draw central vertical axis
        // ----|-----       ----|-----
        // ----|-----       ----xx----
        // ---x|-x---       ---xxxx---
        // --xx|-xx--       --xxxxxx--
        // ====o=====  ...  ====xx====
        // ----|-----       ----xx----
        // --xx|-xx--       --xxxxxx--
        // ---x|-x---       ---xxxx---
        // ----|-----       ----xx----
        // ----|-----       ----|-----
        let mut added = HashSet::new();
        for x in 0..w {
            for y in (-w - r + 1)..=r {
                added.insert((x, y));
            }
        }
        points.extend(added);

        // Draw central horizontal axis
        // ----|-----       ----|-----
        // ----xx----       ----xx----
        // ---xxxx---       ---xxxx---
        // --xxxxxx--       --xxxxxx--
        // ====xx====  ...  =xxxxxxxx=
        // ----xx----       -xxxxxxxx-
        // --xxxxxx--       --xxxxxx--
        // ---xxxx---       ---xxxx---
        // ----xx----       ----xx----
        // ----|-----       ----|-----
        let mut added = HashSet::new();
        for x in -r..(w + r) {
            for y in 0..w {
                added.insert((x, -y));
            }
        }
        points.extend(added);

        // Center range on specified coords
        let points = HashSet::from_iter(
            points
                .iter()
                .map(|pt| (pt.0 + top_left.0, pt.1 + top_left.1)),
        );

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

        TowerRange {
            points,
            path_intersections,
        }
    }
}
