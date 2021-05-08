import { createContext, useState, useContext, useEffect } from 'react';

const context = createContext<any>(null);

export const useInitRustGrid = () => {
    const [grid, setGrid] = useState<any>(null);

    useEffect(() => {
        (async () => {
            try {
                const wasm = await import('rust-grid');
                setGrid(wasm);
            } catch (e) {
                console.error(e);
                setGrid(null);
            }
        })();
    }, []);

    return [context, grid];
};

export const useRustGrid = () => useContext(context);