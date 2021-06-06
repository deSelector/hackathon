import React, { useCallback, useEffect } from "react";

export const useWheelEvent = (ref: React.RefObject<HTMLElement>, callback: (ev: WheelEvent) => void) => {
  const listener = useCallback((ev: WheelEvent) => callback?.(ev), [callback]);
  useEffect(() => {
    ref.current?.addEventListener("wheel", listener);
    return () => ref.current?.removeEventListener("wheel", listener);
  }, [ref, listener]);
};
