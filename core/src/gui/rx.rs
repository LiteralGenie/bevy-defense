use bevy::ecs::system::{ Query, SystemState };
use bevy::ecs::world::World;
use js_sys::Function;
use js_sys::Math::random;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue };
use crate::player::{ PlayerGold, PlayerHealth };

use super::console;
use super::utils::get_prop;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub static fromGui: js_sys::Array;
}

fn extract_request(request: JsValue) -> (String, Function, Function) {
    let event_type = get_prop(&request, "type").as_string().unwrap();

    let temp = get_prop(&request, "resolve");
    let resolve = js_sys::Function::from(temp);

    let temp = get_prop(&request, "reject");
    let reject = js_sys::Function::from(temp);

    return (event_type, resolve, reject);
}

pub fn handle_gui_requests(mut world: &mut World) {
    let len = fromGui.length();

    for i in 0..len {
        let req = fromGui.get(i);
        let (event_type, resolve, reject) = extract_request(req);

        let mut result: Option<Result<JsValue, JsValue>> = None;

        match event_type.as_str() {
            "gold" => {
                let mut state: SystemState<(Query<&PlayerGold>,)> = SystemState::new(&mut world);
                let (query,) = state.get_mut(&mut world);
                let gold = query.single();

                result = Some(resolve.call1(&JsValue::null(), &gold.0.into()));
            }
            "health" => {
                let mut state: SystemState<(Query<&PlayerHealth>,)> = SystemState::new(&mut world);
                let (query,) = state.get_mut(&mut world);
                let health = query.single();

                result = Some(resolve.call1(&JsValue::null(), &health.0.into()));
            }
            _ => {
                console::warn2("Unknown event type", event_type.as_str());
                let _ = reject.call0(&JsValue::null());
            }
        }

        match result {
            None => {}
            Some(r) => {
                if r.is_err() {
                    let msg = format!("Failed to process event type {}", event_type.as_str());
                    console::warn(&msg);
                }
            }
        }
    }

    // Remove processed requests
    // So that if new requests came in mid-process (is that possible?), leave them for next time
    // (This replaces the processed requests with undefined and then pops it because wasm-bindgen doesn't expose a 2-arg splice)
    fromGui.splice(0, len, &JsValue::undefined());
    fromGui.shift();
}
