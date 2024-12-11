use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Worker, WorkerOptions, WorkerType};

#[wasm_bindgen]
pub struct NumberEval {
    number: i32,
}

#[wasm_bindgen]
impl NumberEval {
    pub fn new() -> NumberEval {
        NumberEval { number: 0 }
    }

    pub fn is_even(&self) -> bool {
        self.number % 2 == 0
    }

    pub fn set_number(&mut self, number: i32) -> () {
        self.number = number;
    }

    pub fn get_last_number(&self) -> i32 {
        self.number
    }
}

#[wasm_bindgen]
pub fn using_web_worker_in_rust(url: &str) {
    let worker_options = WorkerOptions::new();
    worker_options.set_type(WorkerType::Module);
    let worker_handle = Rc::new(RefCell::new(Worker::new_with_options(url, &worker_options).unwrap()));
    web_sys::console::log_1(&"Created a new worker from within Wasm".into());

    let document = web_sys::window().unwrap().document().unwrap();

    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::new(move || {
        web_sys::console::log_1(&"oninput callback triggered".into());
        let document = web_sys::window().unwrap().document().unwrap();
        let input_field = document
            .get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");
        let input_field = input_field
            .dyn_ref::<web_sys::HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        match input_field.value().parse::<i32>() {
            Ok(number) => {
                let worker_handle = &*worker_handle.borrow();
                let _ = worker_handle.post_message(&number.into());
                persistent_callback_handle = get_on_msg_callback();
                worker_handle.set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
            }
            Err(_) => {
                web_sys::console::log_1(&"Invalid input".into());
            }
        }
    });

    // Attach the closure as `oninput` callback to the input field.
    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    callback.forget();
}

fn get_on_msg_callback() -> Closure<dyn FnMut(web_sys::MessageEvent)> {
    Closure::new(move |event: web_sys::MessageEvent| {
        web_sys::console::log_2(&"Received response: ".into(), &event.data());
        let result = match event.data().as_bool().unwrap() {
            true => "even",
            false => "odd",
        };
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("web-worker-in-rust-result-field")
            .expect("#resultField should exist")
            .dyn_ref::<web_sys::HtmlElement>()
            .expect("#resultField should be a HtmlInputElement")
            .set_inner_text(result);
    })
}