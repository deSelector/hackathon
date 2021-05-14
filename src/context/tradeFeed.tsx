let trade_buffer: ArrayBuffer;
let raw_data: Trade[];

const MAX_ROW_COUNT = 50;
const MIN_ROW_COUNT = 30;

interface Trade {
  price: number;
  size: number;
  time: number;
  dirty: number;
}

export function generateTradeData(data_width: number): Float64Array {
  ///////
  function fill(buffer: ArrayBuffer, data: Trade[]): Float64Array {
    const array = new Float64Array(buffer, 0, data.length * data_width);
    for (let i = 0, c = 0; i < data.length; i++) {
      const v = data[i];
      if (data_width >= 1) array[c++] = v.price;
      if (data_width >= 2) array[c++] = v.size;
      if (data_width >= 3) array[c++] = v.time;
      if (data_width >= 4) array[c++] = v.dirty;
    }
    return array;
  }

  if (!raw_data) {
    trade_buffer =
      trade_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

    raw_data = Array(
      Math.max(MIN_ROW_COUNT, Math.floor(Math.random() * MAX_ROW_COUNT))
    )
      .fill(0)
      .map(
        () =>
          ({
            price: Math.random() * 20,
            size: Math.random() * 5,
            time: Date.now(),
          } as Trade)
      );
  }

  // inject small changes in each cycle
  let index = Math.floor(raw_data.length * Math.random());
  raw_data[index] = {
    price: Math.random() * 20,
    size: Math.random() * 5,
    time: Date.now(),
    dirty: 1,
  };

  return fill(
    trade_buffer,
    raw_data.sort((a, b) => b.time - a.time)
  );
}
