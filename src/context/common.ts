import { Column, ColumnType, NUM_SIZE } from "../core";

export interface RawData {
    [id: string]: string | number;
}

export function fill(buffer: ArrayBuffer, data: RawData[], data_width: number, columns: Column[], getter?: (buff: RawData, col: Column) => number | string): Int8Array {
    const array = new Int8Array(buffer, 0, data.length * data_width);
    const view = new DataView(array.buffer);
    for (let row = 0, offset = 0; row < data.length; row++) {
        try {
            const buff = data[row];
            for (let col of columns) {
                let v = getter?.(buff, col) ?? buff[col.id];
                if (col.col_type === ColumnType.String) {
                    const s = new TextEncoder().encode((v as string ?? "").substring(0, col.size ?? NUM_SIZE));
                    array.set(s, offset);
                    offset += col.size ?? NUM_SIZE;
                } else {
                    console.assert(typeof v === "number");
                    view.setFloat64(offset, v as number ?? 0);
                    offset += NUM_SIZE;
                }
            }
        } catch (error) {
            console.error(
                `${error}, offset=${offset}, buffer-size=${array.length}`
            );
        }
    }
    return array;
}