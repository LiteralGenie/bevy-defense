use bevy::ecs::{query::Changed, system::Query};
use bevy::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::player::{PlayerGold, PlayerHealth};
use crate::states::GamePhase;
use crate::timers::round_timer::RoundTimer;
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

fn update_round(timer: Res<RoundTimer>) {
    updateState("round".into(), timer.0.into());
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
            ),
        );
    }
}
