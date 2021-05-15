import "./index.scss";
import { useInitRustGrid } from "./hooks";
import { DOBComponent, TapeComponent, SolanaComponent } from "./components";
import { DataProvider } from "./context";

function App() {
  const [GridContext, grid] = useInitRustGrid();

  return (
    <DataProvider>
      <GridContext.Provider value={grid}>
        <div className="app">
          <div id="left-panel">
            <SolanaComponent />
          </div>
          <div id="right-panel">
            <DOBComponent id="canvas1" />
            <TapeComponent id="canvas2" />
          </div>
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
