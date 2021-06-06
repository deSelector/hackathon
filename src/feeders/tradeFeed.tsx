import { fill, RawData } from "../context";
import { calcDataWidth, ColumnType, Schema } from "../core";

let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const ROW_COUNT = 30;

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
      highlight: true
    },
    {
      id: "size",
      name: "Size",
      col_type: ColumnType.Number,
      highlight: true
    },
    {
      id: "time",
      name: "Time",
      col_type: ColumnType.Timestamp
    }
  ]
};

export function generateTradeData(): [Int8Array, number, number] {
  const item = () =>
    ({
      price: +(Math.random() * 5).toFixed(3),
      size: Math.floor(Math.random() * 500),
      time: Date.now()
    } as Trade);

  const data_width = calcDataWidth(tradeSchema);

  if (!raw_data) {
    trade_buffer = trade_buffer ?? new ArrayBuffer(ROW_COUNT * data_width);

    raw_data = Array(ROW_COUNT).fill(0).map(item);

    raw_data.sort((a, b) => b.time - a.time);
  }

  // insert a random bunch of trades on each cycle
  const items = Array(Math.floor(Math.random() * 5))
    .fill(0)
    .map(() => item());
  raw_data.unshift(...items);

  raw_data = raw_data.slice(0, ROW_COUNT);

  const array = fill<Trade>(
    trade_buffer, //
    raw_data,
    data_width,
    tradeSchema.cols
  );
  return [array, data_width, raw_data.length];
}
