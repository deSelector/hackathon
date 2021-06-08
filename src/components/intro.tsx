import "./styles.scss";
import classnames from "classnames";
import React from "react";
import { useDataContext, IntroType } from "../context";

export function Intro() {
  const { showIntro, setShowIntro } = useDataContext();

  return (
    <div className={classnames("intro-panel", { showIntro })}>
      <div className={classnames("intro-a", { showIntro: showIntro === IntroType.a })}>
        <h2>What is it</h2>
        <p>
          Extremely high-performance visualization demo of live market data feed supplied by Pyth Network and
          decentralized Solana blockchain.
        </p>

        <h2>Description</h2>
        <p>
          Assemble basic data-delivery portal by merging real-time market data feed (<i>Pyth + Solana</i>) with static
          research (<i>CoinGecko</i>) and augment it with randomly generated high-frequenecy feeds to simulate a typical
          depth of book (DOB) and a Time and Sales windows.
        </p>
        <h2>Technology</h2>
        <ul>
          <li>
            UI: <em>Typescript + React + Hooks</em>{" "}
          </li>
          <li>
            Image Generation: <em>WebAssembly on Rust</em>
          </li>
          <li>
            Data Rendering: <em>canvas-based grids </em>
          </li>
        </ul>
      </div>
      <div className={classnames("intro-b", { showIntro: showIntro === IntroType.b })}>
        <h2>What is it not</h2>
        <ul>
          <li>Not a production-ready app for mass consumption.</li>
          <li>Not an endorsement for Canvas + WebAssembly + Rust as a preferred solution for fast data rendering</li>
          <li>Not meant to save the world from hunger</li>
        </ul>
      </div>
      <button onClick={() => setShowIntro(IntroType.none)}>{"got it"}</button>
    </div>
  );
}
