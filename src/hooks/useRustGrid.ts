import { createContext, useState, useContext, useEffect } from 'react';

const context = createContext<any>(null);

export const useRustGrid = () => {
    const [grid, setGrid] = useState<any>({ wasm: null });

    useEffect(() => {
        (async () => {
            try {
                const wasm = await import('rust-grid');
                setGrid({ wasm });
            } catch (e) {
                console.error(e);
                setGrid({ wasm: null });
            }
        })();
    }, []);

    return [context.Provider, grid];
};

export const useLoadedRustGrid = () => {
    const { wasm } = useContext(context);
    return { wasm };
};