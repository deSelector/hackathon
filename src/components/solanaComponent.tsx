import "./styles.scss";

export interface SolanaComponentProps {}

export function SolanaComponent(props: SolanaComponentProps) {
  return (
    <div className={"solana-wrapper"}>
      <img src={process.env.PUBLIC_URL + "/sol.svg"} alt="solana" />
    </div>
  );
}
