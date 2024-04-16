use bevy::prelude::*;
use std::collections::HashSet;

use crate::towers::{
    components::TowerMarker, systems::TowersInRange,
};

#[derive(Component)]
pub struct SpeedBuffSource {
    pub multiplier: f64,
    pub targets: HashSet<Entity>,
}

impl SpeedBuffSource {
    pub fn new(multiplier: f64) -> Self {
        Self {
            multiplier,
            targets: HashSet::new(),
        }
    }
}

#[derive(Component)]
pub struct SpeedBuffTarget(pub HashSet<Entity>);

pub fn init_speed_buff_target(
    towers: Query<
        Entity,
        (With<TowerMarker>, Without<SpeedBuffTarget>),
    >,
    mut commands: Commands,
) {
    for entity in towers.iter() {
        commands
            .entity(entity)
            .insert(SpeedBuffTarget(HashSet::new()));
    }
}

pub fn apply_speed_buff(
    mut sources: Query<
        (Entity, &mut SpeedBuffSource, &TowersInRange),
        Changed<TowersInRange>,
    >,
    mut targets: Query<&mut SpeedBuffTarget>,
) {
    for (src_entity, mut src, in_range) in sources.iter_mut() {
        // Add buff to new targets
        for tgt_entity in in_range.0.iter() {
            let mut tgt = targets.get_mut(*tgt_entity).unwrap();
            tgt.0.insert(src_entity);

            src.targets.insert(*tgt_entity);
        }

        // Remove buff from old targets
        let mut to_remove = HashSet::<Entity>::new();
        for tgt_entity in src.targets.iter() {
            if !in_range.0.contains(tgt_entity) {
                to_remove.insert(*tgt_entity);

                let mut tgt = targets.get_mut(*tgt_entity).unwrap();
                tgt.0.remove(&src_entity);
            }
        }

        for tgt_entity in to_remove.iter() {
            src.targets.remove(tgt_entity);
        }
    }
}
