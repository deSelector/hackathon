import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";

import { generateBlockData } from "../context";

const UPDATE_FREQ = 2000;

export interface SolanaComponentProps {
  id: string;
}

export function SolanaComponent(props: SolanaComponentProps) {
  const grid = useRustWasm();
  const [freq] = useState<number>(UPDATE_FREQ);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const tick = async () => {
    if (grid) {
      const data_width = grid.Tape.get_data_width();
      const tape = grid.Tape.new(props.id, size.width, size.height);
      const trades = await generateBlockData(data_width);
      grid.paint_tape(tape, trades);
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
      <ResizableCanvas id={props.id} onResize={onResize} />
    </div>
  );
}
