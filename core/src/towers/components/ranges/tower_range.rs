use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Component)]
pub struct TowerRange {
    pub points: HashSet<(i16, i16)>,
    // Map where key is a path id, and values are the dists where this range intersect
    pub path_intersections: HashMap<u8, HashSet<u16>>,
}
