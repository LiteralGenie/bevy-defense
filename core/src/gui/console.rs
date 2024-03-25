use wasm_bindgen::prelude::*;

// todo: use web_sys console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log2(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn warn2(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error2(a: &str, b: &str);
}
