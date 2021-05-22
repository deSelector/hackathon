import "./index.scss";
import { useInitRustWasm } from "./hooks";
import { DOBComponent, TapeComponent, SolanaComponent } from "./components";
import { DataProvider } from "./context";

function App() {
  const [GridContext, wasm] = useInitRustWasm();

  return (
    <DataProvider>
      <GridContext.Provider value={wasm}>
        <div className="app">
          <div id="left-panel">
            <SolanaComponent />
          </div>
          <div id="right-panel">
            <DOBComponent />
            <TapeComponent />
          </div>
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
