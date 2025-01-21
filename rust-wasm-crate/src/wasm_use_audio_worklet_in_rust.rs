use std::collections::VecDeque;
use crate::set_panic_hook::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

#[wasm_bindgen]
pub async fn use_audio_worklet_in_rust(worklet_url: &str, worker_url: &str) {
    set_panic_hook();

    let worker_options = WorkerOptions::new();
    worker_options.set_type(WorkerType::Module);
    let mut queue: VecDeque<Vec<f32>> = VecDeque::new();
    let worker = Worker::new_with_options(worker_url, &worker_options).unwrap();
    let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
        web_sys::console::log_2(&"Queue length: ".into(), &queue.len().into());
        let data = event.data();
        let array = js_sys::Float32Array::new(&data).to_vec();
        queue.push_back(array);
    }) as Box<dyn FnMut(MessageEvent)>);
    worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();

    let audio_context = web_sys::AudioContext::new().unwrap();
    let audio_worklet = audio_context.audio_worklet().unwrap();
    try_into_result(audio_worklet.add_module(worklet_url)).await;
    let microphone_stream = get_audio_device_stream().await;
    let source_node = audio_context
        .create_media_stream_source(&microphone_stream)
        .unwrap();
    let audio_worklet_node = audio_worklet_node(&audio_context, "use-audio-worklet-in-rust-processor");
    let message_port = audio_worklet_node.port().unwrap();
    // 设置 onmessage 事件处理器
    let closure = Closure::wrap(Box::new(move |event: MessageEvent| {
        // 从事件对象中获取消息
        let data = event.data();
        let array = js_sys::Float32Array::new(&data).to_vec();
        // 打印接收到的消息
        web_sys::console::log_2(&"Send message: ".into(), &array.clone().into());
        worker.post_message(&array.into()).unwrap();
    }) as Box<dyn FnMut(_)>);
    message_port.set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    let destination_node = audio_context.destination();
    source_node
        .connect_with_audio_node(&audio_worklet_node)
        .unwrap();
    audio_worklet_node
        .connect_with_audio_node(&destination_node)
        .unwrap();
}

pub async fn try_into_result(r: Result<js_sys::Promise, JsValue>) -> JsValue {
    wasm_bindgen_futures::JsFuture::from(r.unwrap())
        .await
        .unwrap()
}

pub async fn get_audio_device_stream() -> web_sys::MediaStream {
    let media_devices = media_devices();
    let constraints = media_stream_constraints();
    constraints.set_audio(&wasm_bindgen::JsValue::TRUE);
    let result = media_devices.get_user_media_with_constraints(&constraints);
    let value = try_into_result(result).await;
    let result = value.dyn_into::<web_sys::MediaStream>();
    match result {
        Ok(stream) => stream,
        Err(error) => panic!("Error: {:?}", error),
    }
}

pub fn window() -> web_sys::Window {
    match web_sys::window() {
        Some(window) => window,
        None => panic!(
            "Failed to access the window object; ensure this code runs in a browser context."
        ),
    }
}

pub fn navigator() -> web_sys::Navigator {
    window().navigator()
}

pub fn media_devices() -> web_sys::MediaDevices {
    let result = navigator().media_devices();
    match result {
        Ok(media_devices) => media_devices,
        Err(error) => panic!(
            "Cannot retrieve media devices from the navigator. {:?}",
            error
        ),
    }
}

pub fn media_stream_constraints() -> web_sys::MediaStreamConstraints {
    web_sys::MediaStreamConstraints::new()
}

pub fn audio_worklet_node(
    context: &web_sys::BaseAudioContext,
    name: &str,
) -> web_sys::AudioWorkletNode {
    let result = web_sys::AudioWorkletNode::new(context, name);
    match result {
        Ok(node) => node,
        Err(error) => panic!("Failed to create an AudioWorkletNode. {:?}", error),
    }
}
