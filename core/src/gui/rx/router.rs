use super::super::console;
use super::super::utils::get_prop;
use bevy::ecs::world::World;
use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub static pending_commands: js_sys::Array;
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

    (event_type, resolve, reject, data)
}

fn extract_xy(data: JsValue) -> (f32, f32) {
    let x = get_prop(&data, "x").as_f64().unwrap() as f32;
    let y = get_prop(&data, "y").as_f64().unwrap() as f32;
    (x, y)
}

pub fn handle_gui_requests(world: &mut World) {
    let len = pending_commands.length();

    for i in 0..len {
        let req = pending_commands.get(i);
        let (event_type, resolve, reject, data) =
            extract_request(&req);

        let mut result: Option<Result<JsValue, JsValue>> = None;

        match event_type.as_str() {
            "draw_cursor" => {
                let data = {
                    if data.is_null() {
                        None
                    } else {
                        let position = get_prop(&data, "position");
                        let id_tower = get_prop(&data, "id_tower")
                            .as_f64()
                            .unwrap()
                            as u16;
                        Some((extract_xy(position), id_tower))
                    }
                };

                // @todo: Is it necessary to optimize this by only processing the latest draw_cursor event?
                //        Even if the JS is firing events fast enough to pile up multiple in a single frame, is the performance impact significant?
                let did_draw =
                    super::handlers::handle_draw_cursor(world, data);

                result = Some(resolve.call1(
                    &JsValue::null(),
                    &JsValue::from_bool(did_draw),
                ));
            }
            "draw_range" => {
                let id_tower = match data.as_f64() {
                    Some(id) => Some(id as u64),
                    None => None,
                };

                super::handlers::handle_draw_range(world, id_tower);

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::null()),
                );
            }
            "spawn_tower" => {
                let id_tower =
                    get_prop(&data, "id").as_f64().unwrap() as u16;
                let position = {
                    let pos = get_prop(&data, "position");
                    extract_xy(pos)
                };
                super::handlers::handle_spawn_tower(
                    world, id_tower, position,
                );

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::null()),
                );
            }
            "start_round" => {
                super::handlers::handle_start_round(world);

                result = Some(
                    resolve.call1(&JsValue::null(), &JsValue::null()),
                );
            }
            "start_game" => {
                super::handlers::handle_start_game(world);

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
    pending_commands.splice(0, len, &JsValue::undefined());
    pending_commands.shift();
}
