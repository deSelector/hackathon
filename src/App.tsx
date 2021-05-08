import "./index.scss";
import { useInitRustGrid } from "./hooks";
import { GridComponent } from "./components";

function App() {
  const [GridContext, grid] = useInitRustGrid();

  return (
    <GridContext.Provider value={grid}>
      <div className="app">
        <GridComponent id="canvas1" />
        <GridComponent id="canvas2" />
        <GridComponent id="canvas3" />
        <GridComponent id="canvas4" />
      </div>
    </GridContext.Provider>
  );
}

export default App;
