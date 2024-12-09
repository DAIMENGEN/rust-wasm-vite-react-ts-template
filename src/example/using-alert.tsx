import init from "rust-wasm-crate";
import {useEffect} from "react";
export const UsingAlert = () => {
    useEffect(() => {
        init().then(wasm => {
            wasm.using_alert();
        });
    }, []);
    return (
        <></>
    )
}