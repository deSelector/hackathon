import { fill } from "./common";

let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Trade {
  price: number;
  size: number;
  time: number;
}

export function generateTradeData(data_width: number): Float64Array {
  const item = () =>
    ({
      price: +(Math.random() * 5).toFixed(3),
      size: Math.floor(Math.random() * 500),
      time: Date.now(),
    } as Trade);

  if (!raw_data) {
    trade_buffer =
      trade_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

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

  return fill<Trade>(
    trade_buffer,
    raw_data,
    data_width,
    (data: Trade, col: number) => {
      switch (col) {
        case 0:
          return data.price;
        case 1:
          return data.size;
        case 2:
          return data.time;
        default:
          return 0;
      }
    }
  );
}
