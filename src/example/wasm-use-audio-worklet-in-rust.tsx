import * as wasm from "rust-wasm-crate";

export const WasmUseAudioWorkletInRust = () => {
    return (
        <>
            <button onClick={() => {
                const workerUrl = new URL("../js/worker-test.js", import.meta.url);
                const workletUrl = new URL("../js/use-audio-worklet-in-rust.js", import.meta.url);
                wasm.use_audio_worklet_in_rust(workletUrl.href, workerUrl.href).then(console.log);
            }}>
                Click me to start audio worklet
            </button>
        </>
    )
}