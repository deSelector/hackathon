import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustGrid } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";

const UPDATE_FREQ = 75;

const frequencies = [0, 10, 25, 50, 75, 100, 500, 750, 1000];

export interface GridComponentProps {
  id: string;
}

export function GridComponent(props: GridComponentProps) {
  const grid = useRustGrid();
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
        onClick={() => setFreq(value)}
      >
        {value}
      </button>
    ));
  };

  const render = () => {
    const dob = grid?.DOB.new(props.id, size.width, size.height);
    grid?.paint(dob);
  };

  const onResize = ({ width, height }: { width: number; height: number }) => {
    if (size.width !== width || size.height !== height) {
      setSize({ width, height });
    }
  };

  useAnimationFrame(freq, render);

  return (
    <div className={"dob-wrapper"}>
      <div className="frequency-buttons">{buttons()}</div>
      <ResizableCanvas id={props.id} onResize={onResize} />
    </div>
  );
}
