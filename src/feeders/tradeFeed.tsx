import { fill, RawData } from "../context";
import { calcDataWidth, ColumnType, Schema } from "../core";

let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Trade extends RawData {
  price: number;
  size: number;
  time: number;
}

export const tradeSchema: Schema = {
  cols: [
    {
      id: "price",
      name: "Price",
      col_type: ColumnType.Number,
      precision: 5,
    },
    {
      id: "size",
      name: "Size",
      col_type: ColumnType.Number,
    },
    {
      id: "time",
      name: "Time",
      col_type: ColumnType.Timestamp,
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

  const data_width = calcDataWidth(tradeSchema);

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
    tradeSchema.cols
  );
  return [array, data_width];
}
