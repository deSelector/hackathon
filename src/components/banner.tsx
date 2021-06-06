import "./styles.scss";
import classnames from "classnames";
import React from "react";

export function Banner() {
  return (
    <div className="banner">
      <button className={"left"}>{"what is it ?"}</button>
      <img src={process.env.PUBLIC_URL + "/sol2.jpg"} alt="solana" />
      <button className={"right"}>{"what is it not ?"}</button>
    </div>
  );
}
