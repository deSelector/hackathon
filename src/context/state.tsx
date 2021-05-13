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
  const bid_row_count = Math.floor(Math.random() * (max_row_count + 1));
  const ask_row_count = Math.floor(Math.random() * (max_row_count + 1));

  const prices = Array.from(Array(bid_row_count + ask_row_count))
    .map(Math.random)
    .sort();

  return {
    bids: prices
      .slice(0, bid_row_count)
      .reverse()
      .map(
        (price) =>
          ({
            price,
            size: Math.random(),
          } as DOBSide)
      ),
    asks: prices.slice(bid_row_count).map(
      (price) =>
        ({
          price,
          size: Math.random(),
        } as DOBSide)
    ),
  };
}
