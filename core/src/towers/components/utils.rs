use super::{BaseRangeRadius, BasicRangeType};
use crate::towers::{
    attacks,
    config::{AttackTypeConfig, RangeTypeConfig, TowerConfig},
};
use bevy::{ecs::system::SystemState, prelude::*};

pub fn spawn_tower_bundle(
    world: &mut World,
    config: &TowerConfig,
    position: (i16, i16),
) {
    let mut state = SystemState::<Commands>::new(world);
    let mut commands = state.get(world);

    // Base bundle
    let mut entity = commands.spawn((
        super::TowerMarker(config.id),
        super::TowerAttackEnergy { acc: 0, charges: 0 },
        super::TowerPosition::new(position, config.size),
        super::TowerPriority(super::TowerPriorityTypes::FIRST),
    ));

    // Range
    entity.insert(BaseRangeRadius(config.range.radius));
    entity.insert(match config.range.variant {
        RangeTypeConfig::Basic => BasicRangeType,
    });

    // Offense
    if let Some(cfg) = &config.offense {
        entity.insert((
            match cfg.attack {
                AttackTypeConfig::Basic => attacks::BasicAttack,
            },
            super::BaseDamage(cfg.damage),
            super::BaseAttackSpeed(cfg.speed),
        ));
    }

    state.apply(world)
}
