import { fill } from "../context";
import { Column, ColumnType, Schema } from "../core";

let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Trade {
  price: number;
  size: number;
  time: number;
}

export const tradeSchema: Schema = {
  cols: [
    {
      id: 1,
      name: "Price",
      col_type: ColumnType.Number,
      data_offset: 0,
      data_len: 8,
      precision: 5,
    },
    {
      id: 2,
      name: "Size",
      col_type: ColumnType.Number,
      data_offset: 8,
      data_len: 8,
      precision: 0,
    },
    {
      id: 3,
      name: "Time",
      col_type: ColumnType.Timestamp,
      data_len: 8,
      data_offset: 16,
    },
  ],
};

export function generateTradeData(): [Int8Array, number] {
  const item = () =>
    ({
      price: +(Math.random() * 5).toFixed(3),
      size: Math.floor(Math.random() * 500),
      time: Date.now(),
    } as Trade);

  const data_width = tradeSchema.cols.reduce(
    (p, c) => (p += c.data_len ?? 1),
    0
  );

  if (!raw_data) {
    trade_buffer = trade_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width);

    raw_data = Array(
      Math.max(MIN_ROW_COUNT, Math.floor(Math.random() * MAX_ROW_COUNT))
    )
      .fill(0)
      .map(item);

    raw_data.sort((a, b) => b.time - a.time);
  }

  // insert a random bunch of trades on each cycle
  const items = Array(Math.floor(Math.random() * 5))
    .fill(0)
    .map(() => item());
  raw_data.unshift(...items);

  raw_data = raw_data.slice(0, MAX_ROW_COUNT);

  const array = fill<Trade>(
    trade_buffer,
    raw_data,
    data_width,
    tradeSchema.cols,
    (data: Trade, col: Column) => {
      switch (col.id) {
        case 1:
          return data.price;
        case 2:
          return data.size;
        case 3:
          return data.time;
        default:
          return 0;
      }
    }
  );
  return [array, data_width];
}
