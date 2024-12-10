import {useEffect} from "react";
import * as wasm from "rust-wasm-crate";

export const UsingConsoleLog = () => {
    useEffect(() => {
        wasm.using_console_log();
    }, []);

    return (
        <></>
    )
}