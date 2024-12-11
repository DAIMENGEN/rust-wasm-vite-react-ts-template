import * as wasm from "rust-wasm-crate";
import {useEffect} from "react";
export const WasmInWebWorker = () => {

    useEffect(() => {
        const url = new URL("../js/worker.js", import.meta.url);
        console.log("url", url.href);
        wasm.using_web_worker_in_rust(url.href);
    }, []);

    return (
        <div>
            <div>
                <input type="text" id={"inputNumber"}/>
                <div id={"web-worker-in-rust-result-field"}>

                </div>
            </div>
        </div>
    )
}