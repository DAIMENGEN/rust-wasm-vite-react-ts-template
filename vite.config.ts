import {defineConfig} from "vite"
import wasm from "vite-plugin-wasm";
import mkCert from "vite-plugin-mkcert";
import react from "@vitejs/plugin-react";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
    server: {
      host: "127.0.0.1",
      port: 80
    },
    plugins: [
        wasm(),
        react(),
        mkCert(),
        topLevelAwait()
    ],
    optimizeDeps: {
        exclude: ["rust-wasm-crate"]
    }
})

