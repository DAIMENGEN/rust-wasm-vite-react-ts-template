import init from "rust-wasm-crate";
import {useEffect} from "react";
export const JsSnippets = () => {
  useEffect(() => {
      init().then(wasm => {
          const result = wasm.add_js(100000, 200000);
          console.log(result);
      })
  }, []);

  return (
      <></>
  )
}
