import React from "react";

export const useAnimationFrame = (interval: number = 0, callback: () => void) => {
  // Use useRef for mutable variables that we want to persist
  // without triggering a re-render on their change
  const requestRef = React.useRef<number>(0);
  const timeoutRef = React.useRef<any>(0);

  const cancel = () => {
    cancelAnimationFrame(requestRef.current);
    clearTimeout(timeoutRef.current);
  };

  const animate = (time: number) => {
    cancel();
    callback();
    timeoutRef.current = setTimeout(() =>
      requestRef.current = requestAnimationFrame(animate), interval);
  };


  React.useEffect(() => {
    setTimeout(() => requestRef.current = requestAnimationFrame(animate), 25);
    return () => cancel();
  }, [callback, interval]);
};


