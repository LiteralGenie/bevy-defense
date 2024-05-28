use crate::towers::{
    components::TowerModel, events::TowerClickEvent,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
