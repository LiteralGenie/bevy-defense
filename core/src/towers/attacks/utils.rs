use bevy::{prelude::*, utils::hashbrown::HashSet};

use crate::{
    scenario::Scenario,
    towers::{
        components::{TowerPriorityTypes, TowerRange},
        systems::UnitsByDist,
    },
    units::components::UnitPosition,
};

pub fn filter_targets_by_dist(
    targets_by_dist: &Res<UnitsByDist>,
    range: &TowerRange,
) -> HashSet<Entity> {
    let mut targets = HashSet::new();

    for (id_path, dists) in range.path_intersections.iter() {
        let path = targets_by_dist.0.get(&id_path).unwrap();

        for d in dists {
            let targets_at_dist = path.get(&d);
            if let Some(tgts) = targets_at_dist {
                targets.extend(tgts);
            }
        }
    }

    targets
}

pub fn find_target(
    priority: &TowerPriorityTypes,
    candidates: &HashSet<Entity>,
    query: &Query<&UnitPosition>,
    scenario: &Res<Scenario>,
) -> Option<Entity> {
    match priority {
        TowerPriorityTypes::FIRST => candidates
            .iter()
            .min_by_key(|entity| {
                let pos = query.get(**entity).unwrap();
                let path = scenario.paths.get(&pos.id_path).unwrap();
                let rem_dist = path.points.len() as u16 - pos.dist;
                rem_dist
            })
            .copied(),
        // @todo target priorities
        _ => None,
    }
}
