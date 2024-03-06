use bevy::ecs::world::World;
use bevy::math::Vec2;
use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::super::console;
use super::super::utils::get_prop;
use super::handle_draw_cursor::handle_draw_cursor;
use super::handle_spawn_tower::handle_spawn_tower;

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub static guiRequests: js_sys::Array;
}

fn extract_request(
    request: &JsValue,
) -> (String, Function, Function, JsValue) {
    let event_type = get_prop(&request, "type").as_string().unwrap();

    let temp = get_prop(&request, "resolve");
    let resolve = js_sys::Function::from(temp);

    let temp = get_prop(&request, "reject");
    let reject = js_sys::Function::from(temp);

    let data = get_prop(&request, "data");

    return (event_type, resolve, reject, data);
}

fn extract_xy(data: JsValue) -> Vec2 {
    let x = get_prop(&data, "x").as_f64().unwrap();
    let y = get_prop(&data, "y").as_f64().unwrap();
    Vec2::new(x as f32, y as f32)
}

pub fn handle_gui_requests(world: &mut World) {
    let len = guiRequests.length();

    for i in 0..len {
        let req = guiRequests.get(i);
        let (event_type, resolve, reject, data) =
            extract_request(&req);

        let mut result: Option<Result<JsValue, JsValue>> = None;

        match event_type.as_str() {
            "draw_cursor" => {
                let pos: Option<Vec2>;
                if data.is_null() {
                    pos = None
                } else {
                    let position = get_prop(&data, "position");
                    pos = Some(extract_xy(position));
                }

                handle_draw_cursor(world, pos);

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::TRUE),
                );
            }
            "spawn_tower" => {
                handle_spawn_tower(world, extract_xy(data));

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::null()),
                );
            }
            _ => {
                console::error2(
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