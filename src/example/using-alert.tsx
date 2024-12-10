import * as wasm from "rust-wasm-crate";
import {useEffect} from "react";
export const UsingAlert = () => {
    useEffect(() => {
        wasm.using_alert();
    }, []);
    return (
        <></>
    )
}