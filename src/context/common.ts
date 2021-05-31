import { Column } from "../core";

export function fill<T>(buffer: ArrayBuffer, data: T[], data_width: number, columns: Column[], getter: (data: T, col: Column) => number): Int8Array {
    const array = new Int8Array(buffer, 0, data.length * data_width);
    const view = new DataView(array.buffer);
    for (let row = 0, index = 0, dx = 0; row < data.length; row++, index += dx) {
        dx = 0;
        for (let col of columns) {
            let v = getter(data[row], col);
            view.setFloat64(index + col.data_offset, v);
            dx += 8;
        }
    }
    return array;
}