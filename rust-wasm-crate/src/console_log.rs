#[macro_export]
macro_rules! public_console_log {
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
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// 如果没有 #[macro_export]，则该宏只能在当前 crate 中有效，而不能在外部 crate 中被调用。
macro_rules! private_console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

#[wasm_bindgen]
pub fn using_console_log() {
    set_panic_hook();
    public_console_log!("Hello, rust-wasm-crate!");
    public_console_log!("Hello, rust-wasm-crate!", "Hello, rust-wasm-crate!");
    log("Hello, rust-wasm-crate!");
    log_u32(42);
    log_many("Hello, rust-wasm-crate!", "Hello, rust-wasm-crate!");
    private_console_log!("Hello, rust-wasm-crate! {}, {}, {}", 1998, 07, 22);
}