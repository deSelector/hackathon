import { fill } from "../context";
import { ColumnType, Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer = new ArrayBuffer(0);
let quotes = new Map<string, PythQuote>();
let last_update: number = 0;

interface PythQuote {
  name: string;
  price: number;
  confidence: number;
  time: number;
}

enum DataOffset {
  price = 0,
  confidence = 1,
  time = 2,
  name = 3,
}

export const pythSchema: Schema = {
  cols: [
    {
      id: 1,
      name: "Name",
      col_type: ColumnType.String,
      data_offset: DataOffset.name,
      data_width: 1,
    },
    {
      id: 2,
      name: "Price",
      col_type: ColumnType.Number,
      data_offset: DataOffset.price,
      precision: 5,
    },
    {
      id: 3,
      name: "Confidence",
      col_type: ColumnType.Number,
      data_offset: DataOffset.confidence,
      precision: 5,
    },
    {
      id: 4,
      name: "Time",
      col_type: ColumnType.Timestamp,
      data_offset: DataOffset.time,
    },
  ],
};

export async function generatePythData(): Promise<[Float64Array, number]> {
  const item = (name: string, price: number, confidence: number = 0) =>
    ({
      name,
      price,
      confidence,
      time: Date.now(),
    } as PythQuote);

  // todo: move it outside of the loop?
  await init();

  Array.from(priceMap.values())
    .filter((p) => p.time > last_update)
    .map((p) => quotes.set(p.symbol, item(p.symbol, p.price, p.confidence)));

  const data_width = pythSchema.cols.reduce(
    (p, c) => (p += c.data_width ?? 1),
    0
  );
  const data = Array.from(quotes.values());
  const size = quotes.size * data_width * 8;
  if (data_buffer.byteLength < size) {
    data_buffer = new ArrayBuffer(size);
  }

  last_update = Date.now();
  const array = fill<PythQuote>(
    data_buffer,
    data,
    pythSchema.cols.length,
    (data: PythQuote, col: DataOffset) => {
      switch (col) {
        case DataOffset.name:
          return data.name.charCodeAt(0);
        case DataOffset.price:
          return data.price;
        case DataOffset.confidence:
          return data.confidence;
        case DataOffset.time:
          return data.time;
        default:
          throw new Error(`DataOffset unsupported: ${col}`);
      }
    }
  );
  return [array, data_width];
}
