use crate::towers;
use bevy::asset::Assets;
use bevy::ecs::system::{ResMut, SystemState};
use bevy::ecs::world::World;
use bevy::pbr::StandardMaterial;
use bevy::prelude::Commands;
use bevy::render::mesh::Mesh;
use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::console;
use super::utils::get_prop;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub static guiRequests: js_sys::Array;
}

fn extract_request(request: JsValue) -> (String, Function, Function) {
    let event_type = get_prop(&request, "type").as_string().unwrap();

    let temp = get_prop(&request, "resolve");
    let resolve = js_sys::Function::from(temp);

    let temp = get_prop(&request, "reject");
    let reject = js_sys::Function::from(temp);

    return (event_type, resolve, reject);
}

pub fn handle_gui_requests(world: &mut World) {
    let len = guiRequests.length();

    for i in 0..len {
        let req = guiRequests.get(i);
        let (event_type, resolve, reject) = extract_request(req);

        let mut result: Option<Result<JsValue, JsValue>> = None;

        match event_type.as_str() {
            "spawn_tower" => {
                let mut state: SystemState<(
                    Commands,
                    ResMut<Assets<Mesh>>,
                    ResMut<Assets<StandardMaterial>>,
                )> = SystemState::new(world);

                let (commands, meshes, materials) =
                    state.get_mut(world);
                towers::basic_tower::spawn_model(
                    commands, meshes, materials,
                );
                state.apply(world);

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::null()),
                );
            }
            _ => {
                console::warn2(
                    "Unknown event type",
                    event_type.as_str(),
                );
                let _ = reject.call0(&JsValue::null());
            }
        }

        match result {
            None => {}
            Some(r) => {
                if r.is_err() {
                    let msg = format!(
                        "Failed to process event type {}",
                        event_type.as_str()
                    );
                    console::warn(&msg);
                }
            }
        }
    }

    // Remove processed requests
    // So that if new requests came in mid-process (is that possible?), leave them for next time
    // (This replaces the processed requests with undefined and then pops it because wasm-bindgen doesn't expose a 2-arg splice)
    guiRequests.splice(0, len, &JsValue::undefined());
    guiRequests.shift();
}
