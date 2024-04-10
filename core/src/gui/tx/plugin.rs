use super::utils::{
    set_stat_u32, set_stat_u8, set_string, set_u16, set_u32, set_u64,
    set_u8,
};
use crate::player::{PlayerGold, PlayerHealth};
use crate::states::GamePhase;
use crate::timers::round_timer::RoundTimer;
use crate::timers::tick_timer::TickTimer;
use crate::towers::components::{
    BaseAttackSpeed, BaseDamage, BaseRangeRadius,
    EffectiveAttackSpeed, EffectiveDamage, EffectiveRangeRadius,
};
use crate::towers::config::{AttackTypeConfig, TOWER_CONFIGS};
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
            &BaseRangeRadius,
            &EffectiveRangeRadius,
            Option<&BaseDamage>,
            Option<&EffectiveDamage>,
            Option<&BaseAttackSpeed>,
            Option<&EffectiveAttackSpeed>,
        ),
        Or<(
            Changed<BaseRangeRadius>,
            Changed<EffectiveRangeRadius>,
            Changed<BaseDamage>,
            Changed<EffectiveDamage>,
            Changed<BaseAttackSpeed>,
            Changed<EffectiveAttackSpeed>,
        )>,
    >,
) {
    for (e, br, er, bd, ed, bs, es) in query.iter() {
        let update = js_sys::Object::new();

        let _ = set_u64(&update, "id", e.to_bits());

        set_stat_u8(&update, "range", br.0, er.0);

        match (bd, ed) {
            (Some(bd), Some(ed)) => {
                set_stat_u32(&update, "damage", bd.0, ed.0);
            }
            _ => {}
        }

        match (bs, es) {
            (Some(bs), Some(es)) => {
                set_stat_u8(&update, "attack_speed", bs.0, es.0);
            }
            _ => {}
        }

        updateState("towers".into(), update.into());
    }
}

pub fn update_tower_types() {
    for cfg in TOWER_CONFIGS {
        let update = js_sys::Object::new();

        let _ = set_u16(&update, "id", cfg.id);
        let _ = set_u8(&update, "range_radius", cfg.range.radius);

        if let Some(cfg) = &cfg.offense {
            let _ = set_string(
                &update,
                "attack",
                match cfg.attack {
                    AttackTypeConfig::Basic => "basic",
                },
            );
            let _ = set_u32(&update, "damage", cfg.damage);
            let _ = set_u8(&update, "speed", cfg.speed);
        }

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
