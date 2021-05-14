import "./index.scss";
import { useInitRustGrid } from "./hooks";
import { DOBComponent } from "./components";
import { DataProvider } from "./context";

function App() {
  const [GridContext, grid] = useInitRustGrid();

  return (
    <DataProvider>
      <GridContext.Provider value={grid}>
        <div className="app">
          <DOBComponent id="canvas1" />
          <DOBComponent id="canvas2" />
          {/* <DOBComponent id="canvas3" />
          <DOBComponent id="canvas4" /> */}
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
