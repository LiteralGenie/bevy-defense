use crate::{
    scenario::Scenario,
    towers::{
        components::{
            BaseAttackSpeed, BaseDamage, BaseRangeRadius,
            BasicRangeType, EffectiveAttackSpeed, EffectiveDamage,
            EffectiveRangeRadius, TowerMarker, TowerPosition,
        },
        config::match_config,
    },
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn compute_effective_range(
    query: Query<
        (Entity, &BaseRangeRadius),
        Changed<BaseRangeRadius>,
    >,
    mut commands: Commands,
) {
    for (entity, base_radius) in query.iter() {
        let effective_radius = EffectiveRangeRadius(base_radius.0);
        commands.entity(entity).insert(effective_radius);
    }
}

pub fn compute_basic_range(
    query: Query<
        (Entity, &TowerMarker, &EffectiveRangeRadius, &TowerPosition),
        (With<BasicRangeType>, Changed<EffectiveRangeRadius>),
    >,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    for (entity, id, effective_radius, pos) in query.iter() {
        let range = BasicRangeType::create(
            effective_radius.0,
            pos.top_left,
            match_config(id.0).size,
            &scenario,
        );
        commands.entity(entity).insert(range);
    }
}

pub fn compute_effective_damage(
    query: Query<(Entity, &BaseDamage), Changed<BaseDamage>>,
    mut commands: Commands,
) {
    for (entity, base_damage) in query.iter() {
        let effective_damage = EffectiveDamage(base_damage.0);
        commands.entity(entity).insert(effective_damage);
    }
}

pub fn compute_effective_attack_speed(
    info_query: Query<&BaseAttackSpeed>,
    changed: Query<Entity, Or<(Changed<BaseAttackSpeed>,)>>,
    mut commands: Commands,
) {
    let to_check: HashSet<Entity> =
        HashSet::from_iter(changed.iter());

    for entity in to_check {
        let base = {
            match info_query.get(entity) {
                Ok(res) => res,
                Err(_) => continue,
            }
        };

        let mut update = base.0 as f32;

        // Cap attack speed at once per tick
        update = update.min(100.0);

        commands
            .entity(entity)
            .insert(EffectiveAttackSpeed(update as u8));
    }
}
