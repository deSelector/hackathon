import { useCallback, useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { dobSchema, generateDOBData } from "../feeders";
import React from "react";

const frequencies = [0, 100, 500, 1000, 10000];

const UPDATE_FREQ = 100;

export interface DOBComponentProps {
  id?: string;
}

export function DOBComponent(props: DOBComponentProps) {
  const [id] = useState<string>(props.id ?? "dob-canvas");
  const [rowCount, setRowCount] = useState<number>(0);
  // const { counter, setCounter } = useDataContext();
  const [freq, setFreq] = useState<number>(UPDATE_FREQ);
  const [grid, setGrid] = useState<any>(null);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0
  });

  const wasm = useRustWasm();
  if (wasm && !grid) {
    setGrid(wasm.DOB.new(id, dobSchema));
  }

  const buttons = () => {
    return frequencies.map((value) => (
      <button
        key={value}
        className={classnames({ selected: freq === value })}
        title={`update frequency: ${value} msec`}
        // onDoubleClick={() => setCounter(2)}
        onClick={() => setFreq(value)}
      >
        {value}
      </button>
    ));
  };

  const tick = () => {
    if (grid) {
      const { bids, asks, data_width, count } = generateDOBData();
      setRowCount(count);
      grid.render(bids, asks, data_width, 0, 0, size.width, size.height);
    }
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      grid.set_top_index(0);
      setSize({ width, height });
    }
  };

  useAnimationFrame(freq, tick);

  const onScroll = useCallback(
    ({ top, left }) => {
      if (grid) {
        grid.set_top_index(top);
      }
    },
    [grid]
  );

  return (
    <div className={"dob-wrapper"}>
      <div className="frequency-buttons">{buttons()}</div>
      <ResizableCanvas id={id} onResize={onResize} onScroll={onScroll} rowCount={rowCount} rowHeight={40} />
    </div>
  );
}
