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

#[wasm_bindgen(module = "/js/defined-in-js.js")]
extern "C" {
    fn name() -> String;

    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;

    /// 备注：我们可以看到的是 js 中的 set 方法是没有返回值的，但是这里返回值定了自身类型
    /// 虽然不知道为什么这样写，但是这个代码是可以正常运行的，返回的自身类型也是可以正常使用的
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;

    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn defined_in_js() {
    log(&name());
    let my_class = MyClass::new();
    let _class = my_class.set_number(42);
    log(&format!("Test defined_in_js result: {}", my_class.number()));
    log(&my_class.render());
}