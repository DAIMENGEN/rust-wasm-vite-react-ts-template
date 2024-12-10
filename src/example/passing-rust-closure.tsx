import {useEffect} from "react";
import * as wasm from "rust-wasm-crate";
export const PassingRustClosure = () => {

    useEffect(() => {
        wasm.start_clock();
        wasm.count_button_clicks();
    }, []);


    return (
        <div>
            <button id={"count-button-clicks"}>
                按钮
            </button>
        </div>
    )
}