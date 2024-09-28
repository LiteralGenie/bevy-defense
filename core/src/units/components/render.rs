use bevy::prelude::*;

#[derive(Component)]
pub struct UnitModelMaterials {
    pub materials: Vec<UnitMaterial>,
}

pub struct UnitMaterial {
    pub entity: Entity,
    pub handle: Handle<StandardMaterial>,
    pub initial_color: Color,
    pub damage_color: Color,
}

#[derive(Component)]
pub struct UnitModel {
    pub root: Entity,
    pub base: Entity,
    pub health_bar: Entity,
}
