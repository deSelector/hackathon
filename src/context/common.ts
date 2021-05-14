export function fill<T>(buffer: ArrayBuffer, data: T[], col_count: number, getter: (data: T, col: number) => number): Float64Array {
    const array = new Float64Array(buffer, 0, data.length * col_count);
    for (let r = 0, index = 0; r < data.length; r++) {
        for (let c = 0; c < col_count; c++) {
            array[index++] = getter(data[r], c);
        }
    }
    return array;
}