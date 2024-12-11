import * as wasm from "rust-wasm-crate";
import {useEffect} from "react";
export const RequestAnimationFrame = () => {

    useEffect(() => {
        wasm.using_request_animation_frame();
    }, []);

    return (
        <div>
            <div id="using_request_animation_frame" style={{color: "green"}}>

            </div>
        </div>
    )
}