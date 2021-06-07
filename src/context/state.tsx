export enum IntroType {
  none = 0,
  a = 1,
  b = 2
}

export type State = {
  data: any;
  showIntro: IntroType;
  setShowIntro: (value: IntroType) => void;
  setData: (value: any) => void;
};

export const initialState: State = {
  data: undefined,
  showIntro: IntroType.none,
  setData: (data: any) => null,
  setShowIntro: (value: IntroType) => null
};
