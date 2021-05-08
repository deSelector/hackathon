import { createContext, useState, useContext, useEffect } from 'react';

export const WasmContext = createContext<any>(null);

export const useWasm = () => {
    const [wasmObject, setWasmObject] = useState<any>({ wasm: null });

    useEffect(() => {
        (async () => {
            try {
                const wasm = await import('rust-grid');
                setWasmObject({ wasm });
            } catch (e) {
                console.error(e);
                setWasmObject({ wasm: null });
            }
        })();
    }, []);

    return [WasmContext.Provider, wasmObject];
};

export const useLoadedWasm = () => {
    const { wasm } = useContext(WasmContext);
    return { wasm };
};