import React, { useCallback, useEffect } from "react";

export const useWheelEvent = (
  ref: React.RefObject<HTMLElement>,
  callback: (ev: WheelEvent) => void
) => {
  const wheeled = useCallback((ev: WheelEvent) => callback?.(ev), [callback]);
  useEffect(() => {
    ref.current?.addEventListener("wheel", wheeled);
    return () => ref.current?.removeEventListener("wheel", wheeled);
  }, [ref, wheeled]);
};
