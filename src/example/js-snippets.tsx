import * as wasm from "rust-wasm-crate";
import {useEffect} from "react";

export const JsSnippets = () => {
    useEffect(() => {
        const result = wasm.add_js(100000, 200000);
        console.log(result);
        wasm.defined_in_js();
    }, []);

    return (
        <></>
    )
}
