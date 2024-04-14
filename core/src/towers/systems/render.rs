use crate::{
    animation::components::InterpolateTranslation,
    scenario::Scenario,
    towers::{
        components::{Projectile, TowerModel},
        events::TowerClickEvent,
    },
    units::components::UnitPosition,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

// @todo: Projectiles have no movement prediction
//        They only move towards the unit's position at time of spawn
//        So if the tower is far away, the projectile arrives too late -- behind the unit
pub fn render_attack_start(
    projectile_query: Query<(Entity, &Projectile), Added<Projectile>>,
    pos_query: Query<&UnitPosition>,
    model_query: Query<&Transform>,
    scenario: Res<Scenario>,
    mut commands: Commands,
) {
    const TRAVEL_SPEED: f32 = 0.5;

    for (entity, p) in projectile_query.iter() {
        let model = model_query.get(p.model).unwrap();

        let coord = {
            let pos = pos_query.get(p.unit).unwrap();
            let path = scenario.paths.get(&pos.id_path).unwrap();
            path.points.get(pos.dist as usize).unwrap()
        };
        let unit_pos = Vec3::new(
            coord.pos.0 as f32,
            model.translation.y,
            coord.pos.1 as f32,
        );

        let dist = (model.translation - unit_pos).length();
        let travel_ticks = (dist / TRAVEL_SPEED) as u32;

        commands.entity(entity).insert(InterpolateTranslation::new(
            p.model,
            travel_ticks,
            model.translation,
            unit_pos,
        ));
    }
}

pub fn render_attack_end(
    query: Query<(Entity, &Projectile)>,
    mut done: RemovedComponents<InterpolateTranslation>,
    mut commands: Commands,
) {
    for entity in done.read() {
        if let Ok((entity, p)) = query.get(entity) {
            // Despawn projectile model
            commands.entity(p.model).despawn();

            // Despawn projectile
            commands.entity(entity).despawn();
        }
    }
}

pub fn render_event_handlers(
    query: Query<(Entity, &TowerModel), Added<TowerModel>>,
    mut commands: Commands,
) {
    for (entity, model) in query.iter() {
        commands.entity(model.0).insert((
            PickableBundle::default(),
            On::<Pointer<Click>>::run(
                move |mut writer: EventWriter<TowerClickEvent>| {
                    writer.send(TowerClickEvent(entity.clone()));
                },
            ),
        ));
    }
}
