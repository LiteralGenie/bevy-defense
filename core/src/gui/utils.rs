use wasm_bindgen::JsValue;

pub fn get_prop(val: &JsValue, key: &str) -> JsValue {
    let key_val = JsValue::from_str(key);
    return js_sys::Reflect::get(val, &key_val).unwrap();
}
