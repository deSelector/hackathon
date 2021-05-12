import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustGrid } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
import { useDataContext } from "../context";

const UPDATE_FREQ = 75;

const frequencies = [0, 10, 25, 50, 75, 100, 500, 750, 1000];

const ITEM_COUNT = 1000;
const buffer = new ArrayBuffer(ITEM_COUNT * 8);

export interface GridComponentProps {
  id: string;
}

export function GridComponent(props: GridComponentProps) {
  const grid = useRustGrid();
  const { counter, setCounter } = useDataContext();
  const [freq, setFreq] = useState<number>(UPDATE_FREQ);
  const [size, setSize] = useState<{ width?: number; height?: number }>({
    width: 0,
    height: 0,
  });

  const buttons = () => {
    return frequencies.map((value) => (
      <button
        key={value}
        className={classnames({ selected: freq === value })}
        title={`update frequency: ${value} msec`}
        onDoubleClick={() => setCounter(2)}
        onClick={() => setFreq(value)}
      >
        {`${value}/${counter || 0}`}
      </button>
    ));
  };

  const tick = () => {
    const dob = grid?.DOB.new(props.id, size.width, size.height);

    const data = new Float64Array(
      buffer,
      0,
      Math.floor(Math.random() * ITEM_COUNT) + 1
    );
    for (let i = 0; i < data.length; i++) {
      data[i] = Math.random();
    }

    grid?.paint(dob, data);
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      setSize({ width, height });
    }
  };

  useAnimationFrame(freq, tick);

  console.log(`CONTEXT IN GRID: ${counter}`, counter);
  return (
    <div className={"dob-wrapper"}>
      <div className="frequency-buttons">{buttons()}</div>
      <ResizableCanvas id={props.id} onResize={onResize} />
    </div>
  );
}
