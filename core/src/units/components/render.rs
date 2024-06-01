use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModelMaterials {
    pub materials: Vec<UnitMaterial>,
}

pub struct UnitMaterial {
    pub entity: Entity,
    pub initial: Option<Handle<StandardMaterial>>,
    pub damage: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct UnitModel {
    pub root: Entity,
    pub base: Entity,
    pub health_bar: Entity,
}

#[derive(Component)]
pub struct UnitHealthBarModel(pub Entity);

#[derive(Component)]
pub struct UnitDamageTint;
