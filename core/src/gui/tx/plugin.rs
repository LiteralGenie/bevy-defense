use bevy::ecs::{query::Changed, system::Query};
use bevy::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::player::{PlayerGold, PlayerHealth};
use crate::timers::tick_timer::TickTimer;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub fn updateState(key: String, value: JsValue);
}

fn update_gold(query: Query<&PlayerGold, Changed<PlayerGold>>) {
    for update in query.iter() {
        updateState("gold".into(), update.0.into());
    }
}

fn update_health(query: Query<&PlayerHealth, Changed<PlayerHealth>>) {
    for update in query.iter() {
        updateState("health".into(), update.0.into());
    }
}

fn update_time(timer: Res<TickTimer>) {
    updateState("tick".into(), timer.tick.into());
}

pub struct TxPlugin;

impl Plugin for TxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gold, update_health, update_time),
        );
    }
}
