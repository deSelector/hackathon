import { useCallback, useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import React from "react";
import { pythSchema, generatePythData } from "../feeders";
import { CryptoInfo, cryptos } from "../feeders/unirest";
import { Header } from "./header";

const UPDATE_FREQ = 50;

export interface PythComponentProps {
  id?: string;
}

export function PythComponent(props: PythComponentProps) {
  const [id] = useState<string>(props.id ?? "pyth-canvas");
  const [rowCount, setRowCount] = useState<number>(0);
  const [freq] = useState<number>(UPDATE_FREQ);
  const [grid, setGrid] = useState<any>(null);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0
  });

  const wasm = useRustWasm();
  if (wasm && !grid) {
    setGrid(wasm.Grid.new(id, pythSchema));
  }

  const collectSparks = () => {
    if (grid && !grid.has_sparks() && cryptos?.size) {
      grid.set_sparks(
        Array.from(cryptos).reduce(
          (p, [key, value]: [string, CryptoInfo]) => ((p[key] = value.market_data?.sparkline_7d?.price ?? []), p),
          {} as { [key: string]: number[] }
        )
      );
    }
  };

  const tick = async () => {
    if (grid) {
      const [data, data_width, count] = await generatePythData();
      setRowCount(count);
      collectSparks();
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
    <div className={"solana-wrapper"}>
      <img src={process.env.PUBLIC_URL + "/sol2.jpg"} alt="solana" />
      <Header value={freq} title="Market" description={"LIVE Pyth on Solana + CoinGecko"} />
      <ResizableCanvas id={id} onResize={onResize} onScroll={onScroll} rowCount={rowCount} rowHeight={40} />
    </div>
  );
}
