import { Column, ColumnType, NUM_SIZE } from "../core";

export interface RawData {
  [id: string]: any;
}

export function fill<T extends RawData>(
  buffer: ArrayBuffer,
  data: T[],
  totalSize: number,
  columns: Column[],
  getter?: (item: T, col: Column) => number | string
): Int8Array {
  const array = new Int8Array(buffer, 0, data.length * totalSize);
  const view = new DataView(array.buffer);
  for (let row = 0, offset = 0; row < data.length; row++) {
    try {
      const item = data[row];
      for (const col of columns) {
        const v = getter?.(item, col) ?? item[col.id];
        if (col.col_type === ColumnType.String || col.col_type===ColumnType.Sparkline) {
          if (v) {
            const s = new TextEncoder().encode(
              ((v as string) ?? "").substring(0, col.size ?? NUM_SIZE)
            );
            array.set(s, offset);
          }
          offset += col.size ?? NUM_SIZE;
        } else {
          if (v) {
            console.assert(typeof v === "number");
            view.setFloat64(offset, (v as number) ?? 0);
          }
          offset += NUM_SIZE;
        }
      }
    } catch (error) {
      console.error(`${error}, offset=${offset}, buffer-size=${array.length}`);
    }
  }
  return array;
}
