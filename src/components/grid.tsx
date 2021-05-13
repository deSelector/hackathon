import { useState } from "react";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { useRustGrid } from "../hooks";
import { ResizableCanvas } from "./resizableCanvas";
import "./styles.scss";
import classnames from "classnames";
import { useDataContext } from "../context";
import { generateDOBData } from "../context/state";

const UPDATE_FREQ = 75;

const frequencies = [0, 10, 25, 50, 75, 100, 500, 750, 1000];

const MAX_ROW_COUNT = 100;
const COL_SIDE_COUNT = 2;
const CELL_COUNT = MAX_ROW_COUNT * COL_SIDE_COUNT;
const bid_buffer = new ArrayBuffer(CELL_COUNT * 8);
const ask_buffer = new ArrayBuffer(CELL_COUNT * 8);

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

    const raw_data = generateDOBData(MAX_ROW_COUNT);

    const bids = new Float64Array(
      bid_buffer,
      0,
      raw_data.bids.length * COL_SIDE_COUNT
    );

    const asks = new Float64Array(
      ask_buffer,
      0,
      raw_data.asks.length * COL_SIDE_COUNT
    );

    for (let i = 0, c = 0; i < raw_data.bids.length; i++, c += 2) {
      const v = raw_data.bids[i];
      bids[c] = v.price;
      bids[c + 1] = v.size;
    }

    for (let i = 0, c = 0; i < raw_data.asks.length; i++, c += 2) {
      const v = raw_data.asks[i];
      asks[c] = v.price;
      asks[c + 1] = v.size;
    }

    grid?.paint(dob, bids, asks);
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
