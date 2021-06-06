import "./styles.scss";
import classnames from "classnames";
import React from "react";

const frequencies = [0, 100, 500, 1000];

export interface HeaderProps {
  value: number;
  title: string;
  description?: string;
  onChange?: (freq: number) => void;
}

export function Header(props: HeaderProps) {
  return (
    <div className="header-panel">
      <div className="data-source-section">
        <label>{props.title}</label>
        <div>{props.description}</div>
      </div>
      <div className={classnames("frequency-section", { hidden: !props.onChange })}>
        <label>{"delay:"}</label>
        <div className="frequency-buttons">
          {frequencies.map((v) => (
            <button
              key={v}
              className={classnames({ selected: v === props.value })}
              title={`update throttle: ${v} msec`}
              onClick={() => props.onChange?.(v)}
            >
              {v}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
