use bevy::prelude::*;

#[derive(Event)]
pub struct UnitDamageEvent {
    pub unit: Entity,
    pub damage: u32,
}
