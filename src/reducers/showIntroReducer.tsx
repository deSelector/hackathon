import { useReducer } from "react";
import { initialState, IntroType } from "../context/state";
import { Action } from "./action";

const ShowIntroAction = "ShowIntro";

export const showIntroAction = (payload: IntroType): Action<IntroType> => ({
  type: ShowIntroAction,
  payload
});

export let showIntroReducer = (state: IntroType, action: Action<IntroType>): IntroType => action.payload ?? state;

export const useShowIntroReducer = () => useReducer(showIntroReducer, initialState.showIntro);
