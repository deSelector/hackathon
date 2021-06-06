/// <reference path="./../../rustwasm/pkg/rustwasm.d.ts"/>
import React, { useCallback } from "react";
import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { generateTradeData, tradeSchema } from "../feeders";
import { Header } from "./header";

const UPDATE_FREQ = 100;

export interface TapeComponentProps {
  id?: string;
}

export function TapeComponent(props: TapeComponentProps) {
  const [id] = useState<string>(props.id ?? "tape-canvas");
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
    setGrid(wasm.Grid.new(id, tradeSchema));
  }

  const tick = () => {
    if (grid) {
      const [data, data_width, count] = generateTradeData();
      setRowCount(count);
      grid.render(data, data_width, 0, 0, size.width, size.height);
    }
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      grid?.set_top_index(0);
      setSize({ width, height });
    }
  };

  const onScroll = useCallback(({ top, left }) => grid?.set_top_index(top), [grid]);

  useAnimationFrame(freq, tick);

  return (
    <div className={"tape-wrapper"}>
      <Header value={freq} onChange={(v) => setFreq(v)} title={"Trades"} description={"simulation"} />
      <ResizableCanvas id={id} onResize={onResize} onScroll={onScroll} rowCount={rowCount} rowHeight={40} />
    </div>
  );
}
