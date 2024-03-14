use crate::{
    scenario::Scenario,
    units::components::{
        UnitDist, UnitPathId, UnitStatus, UnitStatusTypes,
    },
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

// Units binned by dist (from start of path)
#[derive(Resource)]
pub struct UnitsByDist(
    pub HashMap<u8, HashMap<u16, HashSet<Entity>>>,
);

pub fn index_units_by_dist(
    mut commands: Commands,
    scenario: Res<Scenario>,
    query: Query<(Entity, &UnitStatus, &UnitPathId, &UnitDist)>,
) {
    // Init bins
    let mut by_path = HashMap::new();

    for path in scenario.paths.values() {
        let mut by_dist: HashMap<u16, HashSet<Entity>> =
            HashMap::new();

        for idx in 0..path.points.iter().len() + 1 {
            by_dist.insert(idx as u16, HashSet::new());
        }

        by_path.insert(path.id, by_dist);
    }

    // Populate bins
    for (entity, status, id_path, dist) in query.iter() {
        if !matches!(status.0, UnitStatusTypes::ALIVE) {
            continue;
        }

        let path_bin = by_path.get_mut(&id_path.0).unwrap();
        let mut dist_bin = path_bin.get_mut(&dist.0).unwrap();
        dist_bin.insert(entity);
    }

    commands.insert_resource(UnitsByDist(by_path));
}
