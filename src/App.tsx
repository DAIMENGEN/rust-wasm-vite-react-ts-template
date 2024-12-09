import {useState} from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import {CallJavascriptClosures} from './example/call-javascript-closures';
import {UsingAlert} from "./example/using-alert.tsx";
import {UsingConsoleLog} from "./example/using-console-log.tsx";
import {ReceivingJavascriptClosures} from "./example/receiving-javascript-closures.tsx";
import {JsSnippets} from "./example/js-snippets.tsx";


function App() {
    const [count, setCount] = useState(0);


    return (
        <>
            <div>
                <UsingAlert/>
                <JsSnippets/>
                <UsingConsoleLog/>
                <CallJavascriptClosures/>
                <ReceivingJavascriptClosures/>
                <a href="https://vite.dev" target="_blank">
                    <img src={viteLogo} className="logo" alt="Vite logo"/>
                </a>
                <a href="https://react.dev" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
            </div>
            <h1>Vite + React</h1>
            <div className="card">
                <button onClick={() => setCount((count) => count + 1)}>
                    count is {count}
                </button>
                <p>
                    Edit <code>src/App.tsx</code> and save to test HMR
                </p>
            </div>
            <p className="read-the-docs">
                Click on the Vite and React logos to learn more
            </p>
        </>
    )
}

export default App