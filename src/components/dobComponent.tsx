import { useCallback, useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { dobSchema, generateDOBData } from "../feeders";
import React from "react";
import { Header } from "./header";

const UPDATE_FREQ = 100;

export interface DOBComponentProps {
  id?: string;
}

export function DOBComponent(props: DOBComponentProps) {
  const [id] = useState<string>(props.id ?? "dob-canvas");
  const [rowCount, setRowCount] = useState<number>(0);
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

  const tick = () => {
    if (grid) {
      const { bids, asks, data_width, count } = generateDOBData();
      setRowCount(count);
      grid.render(bids, asks, data_width, 0, 0, size.width, size.height);
    }
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      grid?.set_top_index(0);
      setSize({ width, height });
    }
  };

  useAnimationFrame(freq, tick);

  const onScroll = useCallback(({ top, left }) => grid?.set_top_index(top), [grid]);

  return (
    <div className={"dob-wrapper"}>
      <Header value={freq} onChange={(v) => setFreq(v)} title={"DOB"} description={"Simulation"} />
      <ResizableCanvas
        id={id}
        onResize={onResize}
        onScroll={onScroll}
        rowCount={rowCount}
        rowHeight={grid?.row_height}
      />
    </div>
  );
}
