use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn element() -> web_sys::HtmlDivElement {
    document()
        .get_element_by_id("using_request_animation_frame")
        .expect("no element with id 'using_request_animation_frame'")
        .dyn_into::<web_sys::HtmlDivElement>()
        .expect("should be a div")
}

#[wasm_bindgen]
pub fn using_request_animation_frame() {
    let func = Rc::new(RefCell::new(None));
    let func_clone = func.clone();
    let mut i = 0;
    *func_clone.borrow_mut() = Some(Closure::new(move || {

        if i > 300 {
            element().set_inner_text("Done.");
            let _ = func.borrow_mut().take();
            return;
        }

        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        element().set_inner_text(&text);
        request_animation_frame(func.borrow().as_ref().unwrap());
    }));
    request_animation_frame(func_clone.borrow().as_ref().unwrap());
}