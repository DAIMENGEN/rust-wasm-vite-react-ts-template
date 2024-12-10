use js_sys::Date;
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


#[wasm_bindgen]
pub fn start_clock() {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let body = document.body().expect("document should have a body");
    let div = document.create_element("div").unwrap();
    body.append_child(&div).expect("failed to append child");
    let closure = Closure::<dyn Fn()>::new(move || update(&div));
    window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000).unwrap();
    closure.forget()
}

fn update(current_time: &web_sys::Element) {
    current_time.set_inner_html(&String::from(
        Date::new_0().to_locale_string("zh-CN", &JsValue::undefined()),
    ))
}


#[wasm_bindgen]
pub fn count_button_clicks() {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let button = document
        .get_element_by_id("count-button-clicks")
        .expect("should have #num-clicks on the page");
    let mut clicks = 0;
    let clone = button.clone();
    let closure = Closure::<dyn FnMut()>::new(move || {
        clicks += 1;
        clone.set_text_content(Some(&format!("{} clicks", clicks)));
    });
    button.dyn_ref::<web_sys::HtmlElement>().expect("should have a HtmlElement").set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget()
}