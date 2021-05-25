import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustWasm } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { dobSchema, generateDOBData } from "../feeders";

const frequencies = [0, 50, 100, 250, 500, 750, 1000, 10000];

const UPDATE_FREQ = 250;

export interface DOBComponentProps {
  id?: string;
}

export function DOBComponent(props: DOBComponentProps) {
  const [id] = useState<string>(props.id ?? "dob-canvas");
  // const { counter, setCounter } = useDataContext();
  const [freq, setFreq] = useState<number>(UPDATE_FREQ);
  const [grid, setGrid] = useState<any>(null);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const wasm = useRustWasm();
  if (wasm && !grid) {
    const g = wasm.DOB.new(id, size.width, size.height);
    g.set_schema(dobSchema);
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
      const data_width = wasm.DOB.get_data_width(); // todo: remove later
      const { bids, asks } = generateDOBData(data_width);
      grid.paint(bids, asks);
    }
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      setSize({ width, height });
    }
  };

  useAnimationFrame(freq, tick);

  return (
    <div className={"dob-wrapper"}>
      <div className="frequency-buttons">{buttons()}</div>
      <ResizableCanvas id={id} onResize={onResize} />
    </div>
  );
}
