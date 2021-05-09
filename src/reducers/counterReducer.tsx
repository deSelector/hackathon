import { useReducer } from "react";
import { initialState } from "../context/state";
import { Action } from "./action";

enum CounterAction {
  Increase = "INCREASE",
  Decrease = "DECREASE",
}

export const increaseAction = (payload: number = 1): Action<number> => ({
  type: CounterAction.Increase,
  payload,
});

export const decreaseAction = (payload: number = 1): Action<number> => ({
  type: CounterAction.Decrease,
  payload,
});

export let counterReducer = (state: number, action: Action<number>): number => {
  switch (action.type) {
    case CounterAction.Increase:
      return state + action.payload;
    case CounterAction.Decrease:
      return state - action.payload;
    default:
      return state;
  }
};

export const useCounterReducer = () =>
  useReducer(counterReducer, initialState.counter);
