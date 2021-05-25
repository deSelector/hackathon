import { fill } from "../context";
import { Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer = new ArrayBuffer(0);
let raw_data = new Map<string, Block>();
let last_update: number = 0;

interface Block {
  price: number;
  confidence: number;
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
  const item = (price: number, confidence: number = 0) =>
    ({
      price,
      confidence,
      time: Date.now(),
    } as Block);

  // todo: move it outside of the loop?
  await init();

  Array.from(priceMap.values())
    .filter((p) => p.time > last_update)
    .map((p) => raw_data.set(p.symbol, item(p.price, p.confidence)));

  last_update = Date.now();
  const data = Array.from(raw_data.values());
  const size = raw_data.size * data_width * 8;
  if (data_buffer.byteLength < size) {
    data_buffer = new ArrayBuffer(size);
  }

  return fill<Block>(
    data_buffer,
    data,
    data_width,
    (data: Block, col: number) => {
      switch (col) {
        case 0:
          return data.price;
        case 1:
          return data.confidence;
        case 2:
          return data.time;
        default:
          return 0;
      }
    }
  );
}
