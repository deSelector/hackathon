import "./index.scss";
import { useInitRustWasm } from "./hooks";
import { DOBComponent, TapeComponent, PythComponent, Intro } from "./components";
import { DataProvider } from "./context";
import React from "react";

function App() {
  const [GridContext, wasm] = useInitRustWasm();

  return (
    <DataProvider>
      <GridContext.Provider value={wasm}>
        <div className="app">
          <div id="left-panel">
            <PythComponent />
          </div>
          <div id="right-panel">
            <DOBComponent />
            <TapeComponent />
          </div>
          <Intro />
        </div>
      </GridContext.Provider>
    </DataProvider>
  );
}

export default App;
