use crate::towers::{
    components::{
        EffectiveAttackSpeed, TowerAttackEnergy, TowerMarker,
        TowerModel, TowerPosition,
    },
    config::match_config,
};
use crate::{
    scenario::Scenario,
    units::components::{UnitPosition, UnitStatus, UnitStatusTypes},
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

// Units binned by dist (from start of path)
#[derive(Resource)]
pub struct UnitsByDist(
    pub HashMap<u8, HashMap<u16, HashSet<Entity>>>,
);

pub fn render_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    towers: Query<
        (Entity, &TowerMarker, &TowerPosition),
        Without<TowerModel>,
    >,
) {
    for (entity, id, pos) in towers.iter() {
        let cfg = match_config(id.0);

        let model = (cfg.spawn_model)(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(
                pos.top_left.0 as f32,
                0.0,
                pos.top_left.1 as f32,
            ),
        );

        commands.entity(entity).insert(TowerModel(model));
    }
}

// @todo: It's a little wasteful to run this system if no towers have enough charge to attack this tick
//        Consider benchmarking the performance impact and if necessary, make this lazy somehow
pub fn index_units_by_dist(
    mut commands: Commands,
    scenario: Res<Scenario>,
    query: Query<(Entity, &UnitStatus, &UnitPosition)>,
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
    for (entity, status, pos) in query.iter() {
        if !matches!(status.0, UnitStatusTypes::ALIVE) {
            continue;
        }

        let path_bin = by_path.get_mut(&pos.id_path).unwrap();
        let dist_bin = path_bin.get_mut(&pos.dist).unwrap();
        dist_bin.insert(entity);
    }

    commands.insert_resource(UnitsByDist(by_path));
}

pub fn update_attack_energy(
    mut query: Query<(&mut TowerAttackEnergy, &EffectiveAttackSpeed)>,
) {
    for (mut energy, speed) in query.iter_mut() {
        energy.acc += speed.0;

        while energy.acc >= 100 {
            energy.acc -= 100;
            energy.charges += 1;
        }
    }
}
