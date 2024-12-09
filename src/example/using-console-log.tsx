import {useEffect} from "react";
import init from "rust-wasm-crate";

export const UsingConsoleLog = () => {
    useEffect(() => {
        init().then(wasm => {
            wasm.using_console_log();
        });
    }, []);

    return (
        <></>
    )
}