import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";

import { generateBlockData } from "./../feeders";

const UPDATE_FREQ = 100;

export interface SolanaComponentProps {
  id?: string;
}

export function SolanaComponent(props: SolanaComponentProps) {
  const wasm = useRustWasm();
  const [id] = useState<string>(props.id ?? "block-canvas");
  const [freq] = useState<number>(UPDATE_FREQ);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const tick = async () => {
    if (wasm) {
      const data_width = wasm.Grid.get_data_width();
      const grid = wasm.Grid.new(id, size.width, size.height);
      const data = await generateBlockData(data_width);
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
    <div className={"solana-wrapper"}>
      <img src={process.env.PUBLIC_URL + "/sol2.jpg"} alt="solana" />
      <ResizableCanvas id={id} onResize={onResize} />
    </div>
  );
}
