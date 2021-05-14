import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustGrid } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
// import { useDataContext } from "../context";
import { generateDOBData } from "../context";

const frequencies = [0, 50, 100, 250, 500, 750, 1000, 10000];

const UPDATE_FREQ = 750;

export interface TapeComponentProps {
  id: string;
}

export function TapeComponent(props: TapeComponentProps) {
  const grid = useRustGrid();
  // const { counter, setCounter } = useDataContext();
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
        // onDoubleClick={() => setCounter(2)}
        onClick={() => setFreq(value)}
      >
        {value}
      </button>
    ));
  };

  const tick = () => {
    if (grid) {
      const data_width = grid.DOB.get_data_width();
      const dob = grid.DOB.new(props.id, size.width, size.height);
      const { bids, asks } = generateDOBData(data_width);

      grid.paint(dob, bids, asks);
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
      <ResizableCanvas id={props.id} onResize={onResize} />
    </div>
  );
}
