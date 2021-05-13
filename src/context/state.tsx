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

export interface DOBData {
  bids: DOBSide[];
  asks: DOBSide[];
}
export interface DOBSide {
  price: number;
  size: number;
}

export function generateDOBData(max_row_count: number): DOBData {
  const prices = Array(Math.floor(Math.random() * (2 * max_row_count + 1)))
    .fill(0)
    .map(() => Math.random() * 20.0)
    .sort((a, b) => a - b);

  const bid_count = Math.floor(prices.length / 2);

  return {
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
          size: Math.random(),
        } as DOBSide)
    ),
  };
}
