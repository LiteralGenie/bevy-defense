use bevy::ecs::{
    query::{Added, Changed},
    system::{Query, SystemState},
    world::World,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::player::{PlayerGold, PlayerHealth};

use super::console;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub fn updateState(key: String, value: JsValue);
}

pub fn update_gold(query: Query<&PlayerGold, Changed<PlayerGold>>) {
    for update in query.iter() {
        updateState("gold".into(), update.0.into());
    }
}

pub fn update_health(
    query: Query<&PlayerHealth, Changed<PlayerHealth>>,
) {
    for update in query.iter() {
        updateState("health".into(), update.0.into());
    }
}
