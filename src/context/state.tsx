let bid_buffer: ArrayBuffer;
let ask_buffer: ArrayBuffer;

const MAX_ROW_COUNT = 100;
const MIN_ROW_COUNT = 30;

export type State = {
  data: any;
  counter: number;
  setData: (value: any) => void;
  setCounter: (value: number) => void;
};

export const initialState: State = {
  data: undefined,
  counter: 0,
  setData: (data: any) => null,
  setCounter: (value: number) => null,
};

interface DOBSide {
  price: number;
  size: number;
}

export function generateDOBData(
  data_width: number
): { bids: Float64Array; asks: Float64Array } {
  ///////
  function fill(buffer: ArrayBuffer, data: DOBSide[]): Float64Array {
    const array = new Float64Array(buffer, 0, data.length * data_width);
    for (let i = 0, c = 0, sum = 0; i < data.length; i++) {
      const v = data[i];
      if (data_width >= 1) array[c++] = v.price;
      if (data_width >= 2) array[c++] = v.size;
      if (data_width >= 3) array[c++] = sum += v.size;
    }
    return array;
  }

  bid_buffer = bid_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);
  ask_buffer = ask_buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

  const prices = Array(
    Math.max(MIN_ROW_COUNT * 2, Math.floor(Math.random() * 2 * MAX_ROW_COUNT))
  )
    .fill(0)
    .map(() => Math.random() * 20.0)
    .sort((a, b) => a - b);

  const bid_count = Math.floor(prices.length / 2);

  const raw_data = {
    bids: prices
      .slice(0, bid_count)
      .reverse()
      .map(
        (price) =>
          ({
            price,
            size: Math.random() * 5.0,
          } as DOBSide)
      ),

    asks: prices.slice(bid_count).map(
      (price) =>
        ({
          price,
          size: Math.random() * 5.0,
        } as DOBSide)
    ),
  };

  return {
    bids: fill(bid_buffer, raw_data.bids),
    asks: fill(ask_buffer, raw_data.asks),
  };
}
