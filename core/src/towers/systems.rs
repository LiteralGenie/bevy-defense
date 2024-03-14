use crate::{
    scenario::Scenario,
    units::components::{
        UnitDist, UnitPathId, UnitStatus, UnitStatusTypes,
    },
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use super::components::Projectile;

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

pub fn render_attacks(
    projectile_query: Query<(Entity, &Projectile)>,
    unit_query: Query<(&UnitPathId, &UnitDist)>,
    mut model_query: Query<&mut Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    const TRAVEL_DIST: f32 = 0.25;
    const DESPAWN_DIST: f32 = 2.0 * TRAVEL_DIST;

    for (entity, p) in projectile_query.iter() {
        let mut model = model_query.get_mut(p.model).unwrap();

        let unit_pos = {
            let (unit_path, unit_dist) =
                unit_query.get(p.unit).unwrap();
            let path = scenario.paths.get(&unit_path.0).unwrap();
            path.points.get(unit_dist.0 as usize).unwrap()
        };

        // Vector pointing to unit position
        let dist = Vec3::new(
            unit_pos.pos.0 as f32 - model.translation.x as f32,
            0.0,
            unit_pos.pos.1 as f32 - model.translation.z as f32,
        );

        if dist.length() > DESPAWN_DIST {
            let update = dist * (TRAVEL_DIST / dist.length());
            model.translation += update;
        } else {
            commands.entity(p.model).despawn();
            commands.entity(entity).despawn();
        }
    }
}
