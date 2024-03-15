use bevy::prelude::*;
use js_sys::JsString;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::player::{PlayerGold, PlayerHealth};
use crate::states::GamePhase;
use crate::timers::round_timer::RoundTimer;
use crate::timers::tick_timer::TickTimer;
use crate::towers::events::TowerClickEvent;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub fn updateState(key: String, value: JsValue);
    pub fn dispatchEvent(name: String, detail: JsValue);
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

fn update_round(timer: Res<RoundTimer>) {
    updateState("round".into(), timer.round.into());
}

fn update_tick(timer: Res<TickTimer>) {
    updateState("tick".into(), timer.0.into());
}

fn update_phase(phase: Res<State<GamePhase>>) {
    updateState(
        "phase".into(),
        match phase.get() {
            GamePhase::INIT => "INIT".into(),
            GamePhase::BUILD => "BUILD".into(),
            GamePhase::COMBAT => "COMBAT".into(),
        },
    );
}

fn handle_tower_click(mut reader: EventReader<TowerClickEvent>) {
    for ev in reader.read() {
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &detail,
            &JsString::from("tower"),
            &JsValue::from_f64(ev.0.to_bits() as f64),
        );
        dispatchEvent("towerclick".into(), JsValue::from(detail));
    }
}

pub struct TxPlugin;

impl Plugin for TxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gold,
                update_health,
                update_round,
                update_tick,
                update_phase,
                handle_tower_click,
            ),
        );
    }
}
