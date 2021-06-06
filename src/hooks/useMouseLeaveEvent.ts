import React, { useCallback, useEffect } from "react";

export const useMouseLeaveEvent = (ref: React.RefObject<HTMLElement>, callback: (ev: MouseEvent) => void) => {
  const listener = useCallback((ev: MouseEvent) => callback?.(ev), [callback]);
  useEffect(() => {
    ref.current?.addEventListener("mouseleave", listener);
    return () => ref.current?.removeEventListener("mouseleave", listener);
  }, [ref, listener]);
};
