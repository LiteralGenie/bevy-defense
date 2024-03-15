use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct TowerClickEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for TowerClickEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self(value.target)
    }
}
