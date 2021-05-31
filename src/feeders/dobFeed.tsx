import { fill, RawData } from "../context";
import { calcDataWidth, Column, ColumnType, Schema } from "../core";

let bid_buffer: ArrayBuffer;
let ask_buffer: ArrayBuffer;
let raw_data: Quote[];

const ROW_COUNT = 30;
const CUM_SIZE_COL_ID = "cumSize"; // don't change it - used by the grid component

interface Quote extends RawData {
  price: number;
  size: number;
  time: number;
}

const sizeCol = {
  id: "size",
  name: "Size",
  col_type: ColumnType.Number,
  precision: 3,
} as Column;

const priceCol = {
  id: "price",
  name: "Price",
  col_type: ColumnType.Number,
  precision: 5,
} as Column;

const cumSizeCol = {
  id: CUM_SIZE_COL_ID,
  name: "CumSize",
  col_type: ColumnType.Number,
  hidden: true,
} as Column;

const timeCol = {
  id: "time",
  name: "Time",
  col_type: ColumnType.Timestamp,
  hidden: true,
} as Column;

export const bidSchema: Schema = {
  cols: [sizeCol, priceCol, cumSizeCol, timeCol],
};
export const askSchema: Schema = {
  cols: [priceCol, sizeCol, cumSizeCol, timeCol],
};

export function generateDOBData(): {
  bids: Int8Array;
  asks: Int8Array;
  data_width: number;
} {
  const item = () =>
    ({
      price: Math.random() * 20,
      size: Math.random() * 5,
      time: Date.now(),
    } as Quote);

  const data_width = calcDataWidth(bidSchema);

  if (!raw_data) {
    bid_buffer = bid_buffer ?? new ArrayBuffer(ROW_COUNT * data_width);
    ask_buffer = ask_buffer ?? new ArrayBuffer(ROW_COUNT * data_width);

    raw_data = Array(ROW_COUNT * 2)
      .fill(0)
      .map(item);
  }

  function toBuffer(buffer: ArrayBuffer, data: Quote[]) {
    let sum = 0;
    return fill(
      buffer,
      data,
      data_width,
      bidSchema.cols,
      (data: RawData, col: Column) => {
        const v = data[col.id];
        return col.id === CUM_SIZE_COL_ID ? (sum += v as number) : v;
      }
    );
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
    data_width,
  };
}
