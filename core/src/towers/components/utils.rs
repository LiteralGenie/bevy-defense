use super::{BaseRangeRadius, BasicRangeType};
use crate::towers::{
    attacks,
    config::{
        AttackTypeConfig, RangeTypeConfig, TowerConfig,
        TowerVariantConfig,
    },
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
            super::BaseDamage(cfg.damage),
            super::BaseAttackSpeed(cfg.speed),
        ));

        match cfg.attack {
            AttackTypeConfig::Basic => {
                entity.insert(attacks::BasicAttack)
            }
            AttackTypeConfig::Aoe(radius) => {
                entity.insert(attacks::AoeAttack(radius))
            }
        };
    }

    // Special cases
    match config.variant {
        TowerVariantConfig::Basic => {}
        TowerVariantConfig::SpeedBuff(val) => {
            entity.insert(attacks::SpeedBuffSource::new(val));
        }
    }

    state.apply(world)
}
