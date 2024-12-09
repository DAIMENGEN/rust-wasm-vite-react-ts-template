use wasm_bindgen::prelude::*;
use crate::set_panic_hook::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn using_alert() {
    set_panic_hook();
    alert("Hello, rust-wasm-crate!");
}