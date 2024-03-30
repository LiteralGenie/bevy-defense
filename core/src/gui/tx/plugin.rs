use super::utils::{set_u16, set_u32, set_u64, set_u8};
use crate::player::{PlayerGold, PlayerHealth};
use crate::states::GamePhase;
use crate::timers::round_timer::RoundTimer;
use crate::timers::tick_timer::TickTimer;
use crate::towers::components::{
    BaseAttackSpeed, BaseDamage, BaseRangeRadius,
    EffectiveAttackSpeed, EffectiveDamage, EffectiveRangeRadius,
};
use crate::towers::config::TOWER_CONFIGS;
use crate::towers::events::TowerClickEvent;
use bevy::prelude::*;
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
            &BaseAttackSpeed,
            &EffectiveAttackSpeed,
        ),
        Or<(
            Changed<BaseDamage>,
            Changed<EffectiveDamage>,
            Changed<BaseRangeRadius>,
            Changed<EffectiveRangeRadius>,
            Changed<BaseAttackSpeed>,
            Changed<EffectiveAttackSpeed>,
        )>,
    >,
) {
    for (e, bd, ed, br, er, bs, es) in query.iter() {
        let update = js_sys::Object::new();

        let _ = set_u64(&update, "id", e.to_bits());

        let _ = set_u32(&update, "base_damage", bd.0);
        let _ = set_u32(&update, "effective_damage", ed.0);

        let _ = set_u8(&update, "base_range", br.0);
        let _ = set_u8(&update, "effective_range", er.0);

        let _ = set_u8(&update, "base_attack_speed", bs.0);
        let _ = set_u8(&update, "effective_attack_speed", es.0);

        updateState("towers".into(), update.into());
    }
}

pub fn update_tower_types() {
    for cfg in TOWER_CONFIGS {
        let update = js_sys::Object::new();

        let _ = set_u16(&update, "id", cfg.id);
        let _ = set_u32(&update, "damage", cfg.damage);
        let _ = set_u8(&update, "speed", cfg.speed);
        let _ = set_u8(&update, "range_radius", cfg.range_radius);

        updateState("tower_types".into(), update.into());
    }
}

/**
 * Emit mouse clicks as window events to gui
 *
 * If the click was targeting a game entity,
 * the entity id will be included in the event details
 */
fn handle_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    mut tower_clicks: EventReader<TowerClickEvent>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    let detail = js_sys::Object::new();

    if let Some(ev) = tower_clicks.read().nth(0) {
        let _ = set_u64(&detail, "tower", ev.0.to_bits());
    }

    dispatchEvent("gameclick".into(), JsValue::from(detail));
}

pub struct TxPlugin;

impl Plugin for TxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (update_tower_types,)).add_systems(
            Update,
            (
                update_gold,
                update_health,
                update_round,
                update_tick,
                update_phase,
                update_towers,
                handle_clicks,
            ),
        );
    }
}
