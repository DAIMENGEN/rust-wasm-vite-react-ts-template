import {useEffect} from "react";
import * as wasm from "rust-wasm-crate";

export const CallJavascriptClosures = () => {
    useEffect(() => {
        wasm.call_javascript_closures0(() => {
            console.log("123456");
        });
        wasm.call_javascript_closures1((a: string) => {
            console.log(a);
        });
    }, []);

    return (
        <div></div>
    )
}