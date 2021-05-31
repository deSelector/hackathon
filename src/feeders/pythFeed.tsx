import { fill } from "../context";
import { calcDataWidth, ColumnType, Schema } from "../core";
import { init, priceMap } from "./pythBridge";

let data_buffer = new ArrayBuffer(0);

export const pythSchema: Schema = {
  cols: [
    {
      id: "symbol",
      name: "Name",
      col_type: ColumnType.String,
      size: 10,
    },
    {
      id: "price",
      name: "Price",
      col_type: ColumnType.Number,
      precision: 5,
    },
    {
      id: "asset",
      name: "Asset",
      col_type: ColumnType.String,
    },
    {
      id: "time",
      name: "Time",
      col_type: ColumnType.Timestamp,
      hidden: true,
    },
  ],
};

export async function generatePythData(): Promise<[Int8Array, number]> {
  // todo: move it outside of the loop?
  await init();

  const data_width = calcDataWidth(pythSchema);
  const quotes = Array.from(priceMap.values());
  const size = quotes.length * data_width;
  if (data_buffer.byteLength < size) {
    data_buffer = new ArrayBuffer(size);
  }

  const array = fill(data_buffer, quotes, data_width, pythSchema.cols);

  return [array, data_width];
}
