import { fill } from "./common";

let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Trade {
  price: number;
  size: number;
  time: number;
  dirty: number;
}

export function generateTradeData(data_width: number): Float64Array {
  if (!raw_data) {
    trade_buffer =
      trade_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

    raw_data = Array(
      Math.max(MIN_ROW_COUNT, Math.floor(Math.random() * MAX_ROW_COUNT))
    )
      .fill(0)
      .map(
        () =>
          ({
            price: Math.random() * 20,
            size: Math.random() * 5,
            time: Date.now(),
          } as Trade)
      );
    raw_data.sort((a, b) => b.time - a.time);
  }

  // insert a new trade on each cycle
  raw_data.unshift({
    price: Math.random() * 20,
    size: Math.random() * 5,
    time: Date.now(),
    dirty: 1,
  });

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
        case 3:
          return data.dirty;
        default:
          return 0;
      }
    }
  );
}
