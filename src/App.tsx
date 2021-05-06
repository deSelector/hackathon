import "./index.scss";
import { useWasm } from "./hooks/useWasm";
import { RustComponent } from "./components";

function App() {
  const [WasmProvider, wasmObject] = useWasm();

  return (
    <WasmProvider value={wasmObject}>
      <div className="app">
        <RustComponent id="canvas1" />
        <RustComponent id="canvas2" />
        <RustComponent id="canvas3" />
        <RustComponent id="canvas4" />
      </div>
    </WasmProvider>
  );
}

export default App;
