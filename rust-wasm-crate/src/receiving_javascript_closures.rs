use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn call_javascript_closures0(callback: &js_sys::Function) {
    callback.call0(&JsValue::NULL).unwrap();
}

#[wasm_bindgen]
pub fn call_javascript_closures1(callback: &js_sys::Function) {
    callback.call1(&JsValue::NULL, &JsValue::from_str("mengen.dai")).unwrap();
}