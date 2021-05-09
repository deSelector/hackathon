import { createContext, useContext, useState } from "react";
import { increaseAction, useCounterReducer } from "../reducers/counterReducer";
import { initialState, State } from "./state";

const DataContext = createContext<State>(initialState);

export const useDataContext = () => useContext(DataContext);

export function DataProvider({ children }: { children: JSX.Element }) {
  const [data, setData] = useState<any>();
  const [counter, dispatch] = useCounterReducer();
  const setCounter = (v: number) => dispatch(increaseAction(v));

  return (
    <DataContext.Provider value={{ data, setData, counter, setCounter }}>
      {children}
    </DataContext.Provider>
  );
}
