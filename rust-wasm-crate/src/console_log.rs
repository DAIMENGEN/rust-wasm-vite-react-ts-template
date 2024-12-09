#[macro_export]
macro_rules! console_log {
    ($msg:expr) => {{
        web_sys::console::log_1(&($msg).into());
    }};
    ($msg:expr, $value:expr) => {{
        web_sys::console::log_2(&($msg).into(), &($value).into());
    }};
}

use wasm_bindgen::prelude::*;
use crate::set_panic_hook::set_panic_hook;
#[wasm_bindgen]
pub fn using_console_log() {
    set_panic_hook();
    console_log!("Hello, rust-wasm-crate!");
    console_log!("Hello, rust-wasm-crate!", "Hello, rust-wasm-crate!");
}