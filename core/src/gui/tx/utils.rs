use bevy::prelude::*;
use js_sys;
use wasm_bindgen::JsValue;

pub fn set_object(
    target: &js_sys::Object,
    key: &str,
) -> js_sys::Object {
    let obj = js_sys::Object::new();

    js_sys::Reflect::set(&target, &js_sys::JsString::from(key), &obj);

    obj
}

pub fn set_string(
    target: &js_sys::Object,
    key: &str,
    value: &str,
) -> Result<bool, JsValue> {
    js_sys::Reflect::set(
        &target,
        &js_sys::JsString::from(key),
        &js_sys::JsString::from(value),
    )
}

// @todo: how to macro these?

pub fn set_stat_u8(
    target: &js_sys::Object,
    key: &str,
    base: u8,
    effective: u8,
) {
    let obj = set_object(&target, key);
    let _ = set_u8(&obj, "base", base);
    let _ = set_u8(&obj, "effective", effective);
}

pub fn set_stat_u16(
    target: &js_sys::Object,
    key: &str,
    base: u16,
    effective: u16,
) {
    let obj = set_object(&target, key);
    let _ = set_u16(&obj, "base", base);
    let _ = set_u16(&obj, "effective", effective);
}

pub fn set_stat_u32(
    target: &js_sys::Object,
    key: &str,
    base: u32,
    effective: u32,
) {
    let obj = set_object(&target, key);
    let _ = set_u32(&obj, "base", base);
    let _ = set_u32(&obj, "effective", effective);
}

pub fn set_stat_u64(
    target: &js_sys::Object,
    key: &str,
    base: u64,
    effective: u64,
) {
    let obj = set_object(&target, key);
    let _ = set_u64(&obj, "base", base);
    let _ = set_u64(&obj, "effective", effective);
}

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
