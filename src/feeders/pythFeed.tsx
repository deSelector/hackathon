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
      size: 10
    },
    {
      id: "price",
      name: "Price",
      col_type: ColumnType.Number,
      precision: 5,
      highlight: true
    },
    {
      id: "sparkline",
      name: "7d History",
      col_type: ColumnType.Sparkline,
      size: 10
    },
    {
      id: "ath",
      name: "ATH",
      col_type: ColumnType.Number,
      precision: 5
    },
    {
      id: "ath_change_percentage",
      name: "ATH % Chg",
      col_type: ColumnType.Number,
      precision: 1
    },
    {
      id: "market_cap_rank",
      name: "Rank",
      col_type: ColumnType.Number,
      precision: 0
    },
    {
      id: "market_cap",
      name: "Mkt Cap ($B)",
      col_type: ColumnType.Number,
      precision: 3
    },
    {
      id: "max_supply",
      name: "Max Supply (M)",
      col_type: ColumnType.Number,
      precision: 3
    },
    {
      id: "asset",
      name: "Asset",
      col_type: ColumnType.String
    },
    // {
    //   id: "description",
    //   name: "Description",
    //   col_type: ColumnType.String,
    //   size: 20,
    // },
    {
      id: "time",
      name: "Time",
      col_type: ColumnType.Timestamp,
      hidden: true
    }
  ]
};

export async function generatePythData(): Promise<[Int8Array, number, number]> {
  // todo: move it outside of the loop?
  await init();

  const size = calcDataWidth(pythSchema);
  const quotes = Array.from(priceMap.values()).sort(sorter);
  const totalSize = quotes.length * size;
  if (buffer.byteLength < totalSize) {
    buffer = new ArrayBuffer(totalSize);
  }

  const array = fill<PythQuote>(buffer, quotes, size, pythSchema.cols, (data: PythQuote, col: Column) => {
    switch (col.id) {
      case "max_supply":
        return (data.max_supply ?? 0) / 1000000;
      case "market_cap":
        return data.market_cap / 1000000000;
      case "sparkline":
        return data.symbol; // works as the key to the sparks cache
      default:
        return data[col.id];
    }
  });

  return [array, size, quotes.length];
}

function sorter(a: PythQuote, b: PythQuote): number {
  return a.asset && b.asset && a.asset !== b.crypto
    ? a.asset?.localeCompare(b.asset || "")
    : a.symbol?.localeCompare(b.symbol);
}
