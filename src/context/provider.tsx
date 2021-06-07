import { createContext, useContext, useState } from "react";
import { showIntroAction, useShowIntroReducer } from "../reducers/counterReducer";
import { initialState, IntroType, State } from "./state";
import React from "react";

const DataContext = createContext<State>(initialState);
export const useDataContext = () => useContext(DataContext);

export function DataProvider({ children }: { children: JSX.Element }) {
  const [data, setData] = useState<any>();
  const [showIntro, dispatch] = useShowIntroReducer();
  const setShowIntro = (v: IntroType) => dispatch(showIntroAction(v));

  return <DataContext.Provider value={{ data, setData, showIntro, setShowIntro }}>{children}</DataContext.Provider>;
}
