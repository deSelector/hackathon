import { fill } from "../context";
import { Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer: ArrayBuffer;
let raw_data: Block[] = [];

const MAX_ROW_COUNT = 50;

interface Block {
  tranCount: number;
  slot: number;
  time: number;
}

export const blockSchema: Schema = {
  cols: [
    { id: 1, name: "Price" },
    { id: 2, name: "Diff" },
    { id: 3, name: "Time" },
  ],
};

export async function generateBlockData(
  data_width: number
): Promise<Float64Array> {
  const item = (slot: number, tranCount: number = 0) =>
    ({
      tranCount,
      slot,
      time: Date.now(),
    } as Block);

  data_buffer = data_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

  // todo: move it outside of the loop?
  await init();

  raw_data.length = 0;
  Array.from(priceMap.values()).map((p) => raw_data.push(item(p.price, 0)));

  raw_data = raw_data.slice(0, MAX_ROW_COUNT);

  return fill<Block>(
    data_buffer,
    raw_data,
    data_width,
    (data: Block, col: number) => {
      switch (col) {
        case 0:
          return data.slot;
        case 1:
          return data.tranCount;
        case 2:
          return data.time;
        default:
          return 0;
      }
    }
  );
}
