import { fill } from "../context";
import { Column, ColumnType, Schema } from "../core";

let bid_buffer: ArrayBuffer;
let ask_buffer: ArrayBuffer;
let raw_data: Quote[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Quote {
  price: number;
  size: number;
  time: number;
}

const sizeCol = {
  id: 1,
  name: "Size",
  col_type: ColumnType.Number,
  data_offset: 1,
  precision: 3,
} as Column;

const priceCol = {
  id: 2,
  name: "Price",
  col_type: ColumnType.Number,
  data_offset: 0,
  precision: 5,
} as Column;

const cumSizeCol = {
  id: 2,
  name: "CumSize",
  col_type: ColumnType.Number,
  data_offset: 2,
  precision: 0,
  hidden: true,
} as Column;

const timeCol = {
  id: 2,
  name: "Time",
  col_type: ColumnType.Timestamp,
  data_offset: 3,
  hidden: true,
} as Column;

export const bidSchema: Schema = {
  cols: [sizeCol, priceCol, cumSizeCol, timeCol],
};
export const askSchema: Schema = {
  cols: [priceCol, sizeCol, cumSizeCol, timeCol],
};

export function generateDOBData(data_width: number): {
  bids: Float64Array;
  asks: Float64Array;
} {
  const item = () =>
    ({
      price: Math.random() * 20,
      size: Math.random() * 5,
      time: Date.now(),
    } as Quote);

  if (!raw_data) {
    bid_buffer = bid_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);
    ask_buffer = ask_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

    raw_data = Array(
      Math.max(MIN_ROW_COUNT * 2, Math.floor(Math.random() * 2 * MAX_ROW_COUNT))
    )
      .fill(0)
      .map(item);
  }

  function toBuffer(buffer: ArrayBuffer, data: Quote[]) {
    let sum = 0;
    return fill<Quote>(buffer, data, data_width, (data: Quote, col: number) => {
      switch (col) {
        case 0:
          return data.price;
        case 1:
          return data.size;
        case 2:
          return (sum += data.size);
        case 3:
          return data.time;
        default:
          return 0;
      }
    });
  }

  // inject a bunch of changes during each cycle
  const count = Math.floor(Math.random() * 10);
  for (let i = 0; i < count; i++) {
    let index = Math.floor(raw_data.length * Math.random());
    raw_data[index] = item();
  }

  // split data in two halves, one for bids and one for asks
  raw_data.sort((a, b) => a.price - b.price);
  const bid_count = Math.floor(raw_data.length / 2);

  return {
    bids: toBuffer(
      bid_buffer,
      raw_data.slice(0, bid_count).sort((a, b) => b.price - a.price)
    ),
    asks: toBuffer(
      ask_buffer,
      raw_data.slice(bid_count).sort((a, b) => a.price - b.price)
    ),
  };
}
