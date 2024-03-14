use crate::{
    scenario::Scenario,
    units::components::{UnitDist, UnitPathId},
};
use bevy::{prelude::*, utils::HashMap};
use std::collections::{BTreeMap, HashSet};

#[derive(Resource)]
pub struct TargetsByDist {
    // Units binned by dist (to end of path, not UnitDist)
    pub units: BTreeMap<u16, HashSet<Entity>>,
    // Last known value for each unit
    value_cache: HashMap<Entity, u16>,
}

pub fn index_units_by_dist(
    mut targets: ResMut<TargetsByDist>,
    updated: Query<
        (Entity, &UnitPathId, &UnitDist),
        Changed<UnitDist>,
    >,
    scenario: Res<Scenario>,
    mut removed: RemovedComponents<UnitDist>,
) {
    for (entity, id_path, dist) in updated.iter() {
        let path = scenario.paths.get(&id_path.0).unwrap();
        let rem_dist = path.points.len() as u16 - 1 - dist.0;

        // Remove unit from existing bin
        match targets.value_cache.get(&entity) {
            Some(prev_dist) => {
                let prev_dist = prev_dist.clone();
                let bin = targets.units.get_mut(&prev_dist).unwrap();
                bin.remove(&entity);
            }
            _ => {}
        }

        // Create bin for new value
        if let None = targets.units.get(&rem_dist) {
            targets.units.insert(rem_dist, HashSet::new());
        }

        // Insert into bin based on new value
        let bin = targets.units.get_mut(&rem_dist).unwrap();
        bin.insert(entity);
    }

    // Remove deleted units from index
    for entity in removed.read() {
        let prev_dist = match targets.value_cache.remove(&entity) {
            None => continue,
            Some(val) => val,
        };

        let bin = match targets.units.get_mut(&prev_dist) {
            None => continue,
            Some(val) => val,
        };

        // Note: empty bins are intentionally left alone
        //       bins represent points on a path and paths last the whole game
        //       (but bins aren't preallocated because... they aren't.)
        bin.remove(&entity);
    }
}
