use wasm_bindgen::prelude::*;

// 注释，这里的 module 的路径必须以 / 开头，以 ./ 或者 ../ 的还没有进行支持
#[wasm_bindgen(module = "/js/foo.js")]
extern "C" {
    fn add(a: u32, b: u32) -> u32;
}


#[wasm_bindgen]
pub fn add_js(a: u32, b: u32) -> u32 {
    add(a, b)
}