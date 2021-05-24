/// <reference path="./../../rustwasm/pkg/rustwasm.d.ts"/>

import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { generateTradeData } from "../feeders";
import { Schema } from "../core";

const frequencies = [0, 50, 100, 250, 500, 750, 1000, 10000];

const UPDATE_FREQ = 500;

const schema: Schema = {
  cols: [
    { id: 1, name: "Col 1" },
    { id: 2, name: "Col 2" },
    { id: 3, name: "Col 3" },
  ],
};

export interface TapeComponentProps {
  id?: string;
}

export function TapeComponent(props: TapeComponentProps) {
  const [id] = useState<string>(props.id ?? "tape-canvas");
  // const { counter, setCounter } = useDataContext();
  const [freq, setFreq] = useState<number>(UPDATE_FREQ);
  const [grid, setGrid] = useState<any>(null);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const wasm = useRustWasm();
  if (wasm && !grid) {
    const g = wasm.Grid.new(id, size.width, size.height);
    g.set_schema(schema);
    setGrid(g);
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
      grid.width = size.width;
      grid.height = size.height;
      const data_width = wasm.Grid.get_data_width(); // todo: fix it - from Grid to grid!
      const data = generateTradeData(data_width);
      grid.paint(data);
    }
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      setSize({ width, height });
    }
  };
  useAnimationFrame(freq, tick);

  return (
    <div className={"tape-wrapper"}>
      <div className="frequency-buttons">{buttons()}</div>
      <ResizableCanvas id={id} onResize={onResize} />
    </div>
  );
}
