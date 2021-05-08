import "./index.scss";
import { useWasm } from "./hooks/useWasm";
import { GridComponent } from "./components";

function App() {
  const [WasmProvider, wasmObject] = useWasm();

  return (
    <WasmProvider value={wasmObject}>
      <div className="app">
        <GridComponent id="canvas1" />
        <GridComponent id="canvas2" />
        <GridComponent id="canvas3" />
        <GridComponent id="canvas4" />
      </div>
    </WasmProvider>
  );
}

export default App;
