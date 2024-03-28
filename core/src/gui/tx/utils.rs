use js_sys;
use wasm_bindgen::JsValue;

pub fn set_u8(
    target: &js_sys::Object,
    key: &str,
    value: u8,
) -> Result<bool, JsValue> {
    js_sys::Reflect::set(
        &target,
        &js_sys::JsString::from(key),
        &JsValue::from(value),
    )
}

pub fn set_u16(
    target: &js_sys::Object,
    key: &str,
    value: u16,
) -> Result<bool, JsValue> {
    js_sys::Reflect::set(
        &target,
        &js_sys::JsString::from(key),
        &JsValue::from(value),
    )
}

pub fn set_u32(
    target: &js_sys::Object,
    key: &str,
    value: u32,
) -> Result<bool, JsValue> {
    js_sys::Reflect::set(
        &target,
        &js_sys::JsString::from(key),
        &JsValue::from(value),
    )
}

pub fn set_u64(
    target: &js_sys::Object,
    key: &str,
    value: u64,
) -> Result<bool, JsValue> {
    js_sys::Reflect::set(
        &target,
        &js_sys::JsString::from(key),
        &JsValue::from(value),
    )
}
