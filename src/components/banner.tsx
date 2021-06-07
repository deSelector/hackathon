import "./styles.scss";
import classnames from "classnames";
import React from "react";
import { IntroType, useDataContext } from "../context";

export function Banner() {
  const { showIntro, setShowIntro } = useDataContext();
  return (
    <div className="banner">
      <button className={"left"} onClick={() => setShowIntro(IntroType.a)}>
        {"what is it ?"}
      </button>
      <img src={process.env.PUBLIC_URL + "/sol2.jpg"} alt="solana" />
      <button className={"right"} onClick={() => setShowIntro(IntroType.b)}>
        {"what is it not ?"}
      </button>
    </div>
  );
}
