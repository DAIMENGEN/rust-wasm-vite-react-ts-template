import {useEffect} from "react";

import * as wasm from "rust-wasm-crate";

export const DomHelloWorld = () => {

    useEffect(() => {
        wasm.create_p_element();
    }, []);

    return (
        <></>
    )
}