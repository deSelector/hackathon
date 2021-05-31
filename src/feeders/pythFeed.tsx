import { fill } from "../context";
import { calcDataWidth, Column, ColumnType, Schema } from "../core";
import { init, priceMap, PythQuote } from "./pythBridge";

let buffer = new ArrayBuffer(0);

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
      id: "market_cap_rank",
      name: "Rank",
      col_type: ColumnType.Number,
      precision: 0,
    },
    {
      id: "market_cap",
      name: "Mkt Cap (Bln)",
      col_type: ColumnType.Number,
      precision: 3,
    },
    {
      id: "asset",
      name: "Asset",
      col_type: ColumnType.String,
    },
    {
      id: "description",
      name: "Description",
      col_type: ColumnType.String,
      size: 20,
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

  const size = calcDataWidth(pythSchema);
  const quotes = Array.from(priceMap.values());
  const totalSize = quotes.length * size;
  if (buffer.byteLength < totalSize) {
    buffer = new ArrayBuffer(totalSize);
  }

  const array = fill<PythQuote>(
    buffer,
    quotes,
    size,
    pythSchema.cols,
    (data: PythQuote, col: Column) =>
      col.id === "market_cap" ? data[col.id] / 1000000000 : data[col.id]
  );

  return [array, size];
}
