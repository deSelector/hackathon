import "./index.scss";
import { useInitRustGrid } from "./hooks";
import { GridComponent } from "./components";
import { DataProvider } from "./context";

function App() {
  const [GridContext, grid] = useInitRustGrid();

  return (
    <DataProvider>
      <GridContext.Provider value={grid}>
        <div className="app">
          <GridComponent id="canvas1" />
          <GridComponent id="canvas2" />
          {/* <GridComponent id="canvas3" />
          <GridComponent id="canvas4" /> */}
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
