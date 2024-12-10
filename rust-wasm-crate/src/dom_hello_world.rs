use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_p_element() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let p = document.create_element("p").unwrap();
    p.set_text_content(Some("Hello, World!"));
    body.append_child(&p).unwrap();
}