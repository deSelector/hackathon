import "./index.scss";
import { useInitRustGrid } from "./hooks";
import { DOBComponent, TapeComponent } from "./components";
import { DataProvider } from "./context";

function App() {
  const [GridContext, grid] = useInitRustGrid();

  return (
    <DataProvider>
      <GridContext.Provider value={grid}>
        <div className="app">
          <DOBComponent id="canvas1" />
          <TapeComponent id="canvas2" />
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
