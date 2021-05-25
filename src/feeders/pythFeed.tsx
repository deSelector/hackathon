import { fill } from "../context";
import { ColumnType, Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer = new ArrayBuffer(0);
let raw_data = new Map<string, Block>();
let last_update: number = 0;

interface Block {
  name: string;
  price: number;
  confidence: number;
  time: number;
}

export const blockSchema: Schema = {
  cols: [
    { id: 1, name: "Name", col_type: ColumnType.String, data_offset: 0 },
    { id: 2, name: "Price", col_type: ColumnType.Number, data_offset: 1 },
    { id: 3, name: "Confidence", col_type: ColumnType.Number, data_offset: 2 },
    { id: 4, name: "Time", col_type: ColumnType.Timestamp, data_offset: 3 },
  ],
};

export async function generateBlockData(
  data_width: number
): Promise<Float64Array> {
  const item = (name: string, price: number, confidence: number = 0) =>
    ({
      name,
      price,
      confidence,
      time: Date.now(),
    } as Block);

  // todo: move it outside of the loop?
  await init();

  Array.from(priceMap.values())
    .filter((p) => p.time > last_update)
    .map((p) => raw_data.set(p.symbol, item(p.symbol, p.price, p.confidence)));

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
          return data.name.charCodeAt(0);
        case 1:
          return data.price;
        case 2:
          return data.confidence;
        case 3:
          return data.time;
        default:
          return 0;
      }
    }
  );
}
