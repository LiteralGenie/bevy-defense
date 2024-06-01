use crate::gui::console;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModel {
    pub root: Entity,
    pub base: Entity,
    pub health_bar: Entity,
}

#[derive(Component)]
pub struct UnitHealthBarModel(pub Entity);

#[derive(Component)]
pub struct UnitModelMaterials<T: Asset> {
    pub entity: Entity,
    pub initial: Option<Handle<T>>,
    pub damage: Handle<T>,
}

#[derive(Component)]
pub struct UnitDamageTint;
