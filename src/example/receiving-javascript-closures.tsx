import {useEffect} from "react";
import * as wasm from "rust-wasm-crate";

export const ReceivingJavascriptClosures = () => {
    useEffect(() => {
        // 外部函数 takes_immutable_closure，它接受不可变闭包
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        window.takes_immutable_closure = (func: () => void) => func()
        // 外部函数 takes_mutable_closure，它接受可变闭包
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        window.takes_mutable_closure = (func: () => void) => func()
        wasm.passing_rust_closures();
    }, []);
    return (
        <div></div>
    )
}