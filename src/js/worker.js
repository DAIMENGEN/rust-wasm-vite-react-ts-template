import init, {NumberEval} from "rust-wasm-crate";
await init();
self.onmessage = async event => {
    const num = NumberEval.new()
    num.set_number(event.data)
    self.postMessage(num.is_even());
}