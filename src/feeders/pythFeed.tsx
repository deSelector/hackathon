import { calcDataWidth, Column, ColumnType, NUM_SIZE, Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer = new ArrayBuffer(0);
let quoteMap = new Map<string, PythQuote>();
let last_update: number = 0;

interface PythQuote {
  name: string;
  price: number;
  confidence: number;
  time: number;
}

export const pythSchema: Schema = {
  cols: [
    {
      id: 1,
      name: "Name",
      col_type: ColumnType.String,
      data_offset: 24,
      data_len: 10,
    },
    {
      id: 2,
      name: "Price",
      col_type: ColumnType.Number,
      data_offset: 0,
      precision: 5,
    },
    {
      id: 3,
      name: "Confidence",
      col_type: ColumnType.Number,
      data_offset: 8,
      precision: 5,
    },
    {
      id: 4,
      name: "Time",
      col_type: ColumnType.Timestamp,
      data_offset: 16,
    },
  ],
};

export async function generatePythData(): Promise<[Int8Array, number]> {
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
    .map((p) => quoteMap.set(p.symbol, item(p.symbol, p.price, p.confidence)));

  const data_width = calcDataWidth(pythSchema);
  const quotes = Array.from(quoteMap.values());
  const size = quotes.length * data_width;
  if (data_buffer.byteLength < size) {
    data_buffer = new ArrayBuffer(size);
  }

  last_update = Date.now();

  const array = fill<PythQuote>(
    data_buffer,
    quotes,
    data_width,
    pythSchema.cols,
    (data: PythQuote, col: Column) => {
      switch (col.id) {
        case 1:
          return new TextEncoder().encode(
            data.name.substring(0, col.data_len ?? 0 + 1)
          );

        case 2:
          return data.price;
        case 3:
          return data.confidence;
        case 4:
          return data.time;
        default:
          throw new Error(`col.id unsupported: ${col.id}`);
      }
    }
  );

  return [array, data_width];
}

export function fill<T>(
  buffer: ArrayBuffer,
  quotes: T[],
  data_width: number,
  columns: Column[],
  getter: (data: T, col: Column) => number | Uint8Array
): Int8Array {
  const array = new Int8Array(buffer, 0, quotes.length * data_width);
  const view = new DataView(array.buffer);

  for (
    let row = 0, index = 0, dx = 0;
    row < quotes.length;
    row++, index += dx
  ) {
    const row_data = quotes[row];
    dx = 0;
    for (let col of columns) {
      const v = getter(row_data, col);
      if (typeof v === "number") {
        view.setFloat64(index + col.data_offset, v);
        dx += NUM_SIZE;
      } else {
        console.assert(
          v.length <= (col.data_len || 0),
          `data size too large, max: ${col.data_len}`
        );
        array.set(v, index + col.data_offset);
        dx += col.data_len ?? 0;
      }
    }
  }
  return array;
}
