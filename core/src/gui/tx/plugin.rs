use crate::player::{PlayerGold, PlayerHealth};
use crate::states::GamePhase;
use crate::timers::round_timer::RoundTimer;
use crate::timers::tick_timer::TickTimer;
use crate::towers::components::{
    BaseDamage, BaseRangeRadius, EffectiveDamage,
    EffectiveRangeRadius,
};
use crate::towers::events::TowerClickEvent;
use bevy::prelude::*;
use js_sys::JsString;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

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

fn update_towers(
    query: Query<
        (
            Entity,
            &BaseDamage,
            &EffectiveDamage,
            &BaseRangeRadius,
            &EffectiveRangeRadius,
        ),
        Or<(
            Changed<BaseDamage>,
            Changed<EffectiveDamage>,
            Changed<BaseRangeRadius>,
            Changed<EffectiveRangeRadius>,
        )>,
    >,
) {
    for (e, bd, ed, br, er) in query.iter() {
        let update = js_sys::Object::new();

        let _ = js_sys::Reflect::set(
            &update,
            &JsString::from("id"),
            &JsValue::from(e.to_bits()),
        );

        let _ = js_sys::Reflect::set(
            &update,
            &JsString::from("base_damage"),
            &JsValue::from(bd.0),
        );

        let _ = js_sys::Reflect::set(
            &update,
            &JsString::from("effective_damage"),
            &JsValue::from(ed.0),
        );

        let _ = js_sys::Reflect::set(
            &update,
            &JsString::from("base_range"),
            &JsValue::from(br.0),
        );

        let _ = js_sys::Reflect::set(
            &update,
            &JsString::from("effective_range"),
            &JsValue::from(er.0),
        );

        updateState("towers".into(), update.into());
    }
}

fn handle_tower_click(mut reader: EventReader<TowerClickEvent>) {
    for ev in reader.read() {
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &detail,
            &JsString::from("tower"),
            &JsValue::from(ev.0.to_bits()),
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
                update_towers,
                handle_tower_click,
            ),
        );
    }
}
