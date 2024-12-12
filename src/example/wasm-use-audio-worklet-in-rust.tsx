import * as wasm from "rust-wasm-crate";

export const WasmUseAudioWorkletInRust = () => {
    return (
        <>
            <button onClick={() => {
                const url = new URL("../js/use-audio-worklet-in-rust.js", import.meta.url);
                const href = url.href;
                wasm.use_audio_worklet_in_rust(href).then(console.log);
            }}>
                Click me to start audio worklet
            </button>
        </>
    )
}