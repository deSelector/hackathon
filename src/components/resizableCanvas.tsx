import { useRef } from "react";
import useResizeObserver from "../hooks/useResizeObserver";
import "./styles.scss";
import React from "react";
export interface ResizableCanvasProps {
  id: string;
  onResize: ({ width, height }: { width: number; height: number }) => void;
}

export const ResizableCanvas: React.FC<ResizableCanvasProps> = (
  props: ResizableCanvasProps
) => {
  const { id, onResize } = props;
  const div = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);

  const callback = () => {
    if (div.current && canvas.current) {
      const { clientWidth: width, clientHeight: height } = div.current;
      const ctx = canvas.current.getContext("2d");
      const scale = window.devicePixelRatio;
      canvas.current.width = Math.floor(width * scale);
      canvas.current.height = Math.floor(height * scale);
      canvas.current.style.width = `${width}px`;
      canvas.current.style.height = `${height}px`;

      if (ctx) {
        ctx.font = "16px sans-serif";
        ctx.scale(scale, scale);
      }

      onResize?.({ width, height });
    }
  };

  useResizeObserver({ callback, element: div });

  return (
    <div ref={div} className={"canvas-wrapper"}>
      <canvas ref={canvas} id={id}></canvas>
    </div>
  );
};
