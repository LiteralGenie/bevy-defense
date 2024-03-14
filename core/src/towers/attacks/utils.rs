use bevy::prelude::*;

use crate::{
    towers::{
        components::TowerPriorityTypes, systems::TargetsByDist,
    },
    units::components::{UnitStatus, UnitStatusTypes},
};

pub fn find_target(
    priority: &TowerPriorityTypes,
    targets_by_dist: &Res<TargetsByDist>,
    info_query: &Query<&UnitStatus>,
) -> Option<Entity> {
    match priority {
        TowerPriorityTypes::FIRST => {
            // Iterate over distance bins
            targets_by_dist.units.values().find_map(|bin| {
                // Iterate over units in bin
                bin.iter().find_map(|entity| {
                    // Return first unit that's alive
                    let status = info_query.get(*entity).unwrap();
                    match status.0 {
                        UnitStatusTypes::ALIVE => {
                            Some(entity.clone())
                        }
                        _ => None,
                    }
                })
            })
        }
        _ => None,
    }
}
