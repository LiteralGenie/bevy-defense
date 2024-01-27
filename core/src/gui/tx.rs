use wasm_bindgen::{ prelude::wasm_bindgen };

#[wasm_bindgen(js_namespace = game)]
extern "C" {
    pub fn setGold(gold: u16);
}
