use bevy::prelude::*;

use crate::units::components::{UnitHealth, UnitStatus};

#[derive(Component)]
pub struct BasicAttack {
    damage: u32,
}

pub fn apply_basic_attack(
    query: Query<&BasicAttack>,
    targets: Query<(&UnitStatus, &UnitHealth)>,
) {
}
