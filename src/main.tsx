import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import init from "rust-wasm-crate";

init().then(() => createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <App/>
    </StrictMode>,
))


