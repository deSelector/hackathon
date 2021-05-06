import { useRef } from "react";
import useObserver from "../hooks/useObserver";
import "./styles.scss";

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
      canvas.current.width = width;
      canvas.current.height = height;
      canvas.current.style.width = `${width}px`;
      canvas.current.style.height = `${height}px`;
      onResize?.({ width, height });
    }
  };

  useObserver({ callback, element: div });

  return (
    <div ref={div} className={"canvas-wrapper"}>
      <canvas ref={canvas} id={id}></canvas>
    </div>
  );
};
