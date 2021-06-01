import { createContext, useState, useContext, useEffect } from "react";

const context = createContext<any>(null);

export const useInitRustWasm = () => {
  const [wasm, setWasm] = useState<any>(null);

  useEffect(() => {
    (async () => {
      try {
        const wasm = await import("rustwasm");
        setWasm(wasm);
      } catch (e) {
        console.error(e);
        setWasm(null); 
      }
    })();
  }, []);

  return [context, wasm]; 
};

export const useRustWasm = () => useContext(context);