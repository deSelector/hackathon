import "./styles.scss";
import classnames from "classnames";
import React from "react";

const frequencies = [0, 100, 500, 1000];

export interface DOBComponentProps {
  value: number;
  onChange: (freq: number) => void;
}

export function FreqButtons(props: DOBComponentProps) {
  return (
    <div className="frequency-buttons">
      {frequencies.map((v) => (
        <button
          key={v}
          className={classnames({ selected: v === props.value })}
          title={`update frequency: ${v} msec`}
          onClick={() => props.onChange(v)}
        >
          {v}
        </button>
      ))}
    </div>
  );
}
