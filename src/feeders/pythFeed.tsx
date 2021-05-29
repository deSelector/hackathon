import { Column, ColumnType, Schema } from "../core";
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
      data_offset: 3,
      data_width: 8,
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
      data_offset: 1,
      precision: 5,
    },
    {
      id: 4,
      name: "Time",
      col_type: ColumnType.Timestamp,
      data_offset: 2,
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
    .map((p) => quoteMap.set(p.symbol, item(p.symbol, p.price, p.confidence)));

  const data_width = pythSchema.cols.reduce(
    (p, c) => (p += c.data_width ?? 1),
    0
  );
  const quotes = Array.from(quoteMap.values());
  const size = quoteMap.size * data_width * 8;
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
            data.name.substring(0, col.data_width ?? 1)
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
  data: T[],
  data_width: number,
  columns: Column[],
  getter: (data: T, col: Column) => number | Uint8Array
): Float64Array {
  const array = new Float64Array(buffer, 0, data.length * data_width);
  for (let r = 0, index = 0; r < data.length; r++, index += data_width) {
    const row_data = data[r];
    for (let col of columns) {
      const v = getter(row_data, col);
      if (typeof v === "number") {
        array[index + col.data_offset] = v;
      } else {
        array.set(v, index + col.data_offset);
      }
    }
  }
  return array;
}
