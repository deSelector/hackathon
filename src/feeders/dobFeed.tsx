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

export const dobSchema: Schema = {
  cols: [
    {
      id: "size",
      name: "Size",
      col_type: ColumnType.Number,
      precision: 3,
      highlight: true,
    } as Column,

    {
      id: "price",
      name: "Price",
      col_type: ColumnType.Number,
      precision: 5,
      highlight: true,
    } as Column,

    {
      id: CUM_SIZE_COL_ID,
      name: "CumSize",
      col_type: ColumnType.Number,
      hidden: true,
    } as Column,

    {
      id: "time",
      name: "Time",
      col_type: ColumnType.Timestamp,
      hidden: true,
    } as Column,
  ],
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

  const data_width = calcDataWidth(dobSchema);

  if (!raw_data) {
    bid_buffer = bid_buffer ?? new ArrayBuffer(ROW_COUNT * data_width);
    ask_buffer = ask_buffer ?? new ArrayBuffer(ROW_COUNT * data_width);

    raw_data = Array(ROW_COUNT * 2)
      .fill(0)
      .map(item);
  }

  function toBuffer(buffer: ArrayBuffer, data: Quote[]) {
    let sum = 0;
    return fill<Quote>(
      buffer,
      data,
      data_width,
      dobSchema.cols,
      (data: Quote, col: Column) =>
        col.id === CUM_SIZE_COL_ID ? (sum += data.size) : data[col.id]
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
