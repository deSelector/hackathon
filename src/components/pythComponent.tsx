import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";

import { blockSchema, generateBlockData } from "../feeders";

const UPDATE_FREQ = 50;

export interface PythComponentProps {
  id?: string;
}

export function PythComponent(props: PythComponentProps) {
  const [id] = useState<string>(props.id ?? "block-canvas");
  const [freq] = useState<number>(UPDATE_FREQ);
  const [grid, setGrid] = useState<any>(null);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const wasm = useRustWasm();
  if (wasm && !grid) {
    const g = wasm.Grid.new(id, size.width, size.height);
    g.set_schema(blockSchema);
    setGrid(g);
  }

  const tick = async () => {
    if (grid) {
      grid.width = size.width;
      grid.height = size.height;
      grid.data_width = 4; // todo: 4
      const data = await generateBlockData(4); // todo: 4
      grid.render(data);
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
