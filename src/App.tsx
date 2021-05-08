import "./index.scss";
import { useRustGrid } from "./hooks";
import { GridComponent } from "./components";

function App() {
  const [WasmProvider, wasmObject] = useRustGrid();

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
