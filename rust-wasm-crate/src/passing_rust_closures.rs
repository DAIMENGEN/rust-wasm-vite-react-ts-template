use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn takes_immutable_closure(f: &dyn Fn());

    fn takes_mutable_closure(f: &mut dyn FnMut());
}

#[wasm_bindgen]
pub fn passing_rust_closures() {
    // 调用 takes_immutable_closure 函数，传递不可变闭包
    takes_immutable_closure(&|| {
        // 闭包体
        web_sys::console::log_1(&"This is an immutable closure".into());
    });

    let mut times_called = 0;

    // 调用 takes_mutable_closure 函数，传递可变闭包
    takes_mutable_closure(&mut || {
        times_called += 1;
        web_sys::console::log_1(&format!("The closure has been called {} times", times_called).into());
    });

    // 输出调用次数
    web_sys::console::log_1(&format!("Final count: {}", times_called).into());
}