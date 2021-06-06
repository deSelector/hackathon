import { useCallback, useRef } from "react";
import "./styles.scss";
import React from "react";
import { useWheelEvent, useResizeObserver } from "../hooks";

export interface ResizableCanvasProps {
  id: string;
  rowCount?: number;
  rowHeight?: number;
  onResize: ({ width, height }: { width: number; height: number }) => void;
  onScroll?: ({ top, left }: { top?: number; left?: number }) => void;
}

export const ResizableCanvas: React.FC<ResizableCanvasProps> = (props: ResizableCanvasProps) => {
  const { id, onResize, onScroll } = props;
  const div = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);
  let top = 0;

  const resized = () => {
    if (div.current && canvas.current) {
      onResize?.(doResize(canvas.current, div.current.clientWidth, div.current.clientHeight));
    }
  };

  const wheeled = useCallback(
    (e: WheelEvent) => {
      if (e.deltaY) {
        const delta = Math.sign(e.deltaY) / 2;
        top = Math.max(0, Math.min(top + delta, (props.rowCount || 0) - 1));
        props.onScroll?.({ top });
      }
    },
    [canvas, onScroll, props.rowCount]
  );

  useResizeObserver({ callback: resized, element: div });
  useWheelEvent(canvas, wheeled);

  return (
    <div ref={div} className={"canvas-wrapper"}>
      <canvas ref={canvas} id={id}></canvas>
    </div>
  );
};

function doResize(canvas: HTMLCanvasElement, width: number, height: number): { width: number; height: number } {
  const scale = window.devicePixelRatio;
  canvas.width = Math.floor(width * scale);
  canvas.height = Math.floor(height * scale);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.font = "16px sans-serif";
    ctx.scale(scale, scale);
  }
  return { width, height };
}
