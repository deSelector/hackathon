import { parseMappingData, parsePriceData, parseProductData, } from "@pythnetwork/client";
import { Buffer } from "buffer";
import { AccountInfo, Commitment, Connection, PublicKey } from "@solana/web3.js";
import { RawData } from "../context";
import { cryptos, getCryptos, CryptoInfo } from "./unirest";

const URL = "https://devnet.solana.com";
const ORACLE_MAPPING_PUBLIC_KEY = new PublicKey("ArppEFcsybCLE8CRtQJLQ9tLv2peGmQoKWFuiUWm4KBP");

let conn: Connection;
export const priceMap = new Map<string, PythQuote>();
let pending: Promise<AccountInfo<Buffer> | null>;

export interface PythQuote extends RawData {
    symbol: string;
    description?: string;
    asset?: string;
    price?: number;
    confidence?: number;
    time?: number;
}

const setPrice = (product: any, buffer: Buffer) => {
  const data = parsePriceData(Buffer.from(buffer));
  const { price, confidence } = data;
  const { symbol, description, nasdaq_symbol, cms_symbol, asset_type: asset } = product;

  const delta = {
    symbol: nasdaq_symbol ?? cms_symbol ?? symbol,
    price,
    confidence,
    time: Date.now(),
  } as PythQuote;

  const obj = priceMap.get(delta.symbol);
  if (obj) {
    Object.assign(obj, delta);
  } else {
    const { name, market_cap_rank, market_data } = cryptos.get(delta.symbol) || {} as CryptoInfo;
    priceMap.set(delta.symbol, {
      ...delta,
      description: name ?? description,
      asset, market_cap_rank,
      market_cap: market_data?.market_cap?.usd
    });
  }
};

export async function init(): Promise<any> {
  conn = conn ?? new Connection(URL, "confirmed");
  if (!pending) {

    getCryptos();

    let start = performance.now();
    console.log("BRIDGE: initializing");

    try {
      pending = conn.getAccountInfo(ORACLE_MAPPING_PUBLIC_KEY);
      const mapping = await pending;
      if (mapping) {
        console.log(`BRIDGE: loaded account info, ${(performance.now() - start) | 0} msec`);
        const { productAccountKeys } = parseMappingData(mapping.data);
        const productAccts = await getAccounts(
          conn,
          productAccountKeys.map((a) => a.toBase58()),
          "confirmed"
        );
        console.log(`BRIDGE: loaded ${productAccts.keys.length} accounts, ${(performance.now() - start) | 0} msec`);
        const products = productAccts.values.map((a) =>
          parseProductData(Buffer.from(a.data))
        );
        console.log(`BRIDGE: parsed ${products.length} products, ${(performance.now() - start) | 0} msec`, products);
        const priceAccts = await getAccounts(
          conn,
          products.map((p) => p.priceAccountKey.toBase58()),
          "confirmed"
        );
        priceAccts.keys.forEach((key, i) => {
          const { product } = products[i];
          setPrice(product, priceAccts.values[i].data);
          conn.onAccountChange(new PublicKey(key), (acc) =>
            setPrice(product, acc.data)
          );
        });
        console.log(`BRIDGE: subscribed to ${priceAccts.keys.length} instruments, ${(performance.now() - start) | 0} msec`);
      }
    } catch (error) {
      console.error(error);
    }
  }

}

const getAccounts = async (connection: Connection, keys: string[], commitment: Commitment) => {
  const accounts = await Promise.all(chunks(keys, 99)
    .map((chunk) => getAccountsCore(connection, chunk, commitment)));

  return {
    keys,
    values: accounts.map(a =>
      a.values.filter(_ => _)
        .map(acc => {
          const { data, ...rest } = acc;
          return {
            ...rest,
            data: Buffer.from(data[0], "base64"),
          } as AccountInfo<Buffer>;
        })
    ).flat()
  };
};

const getAccountsCore = async (connection: Connection, keys: string[], commitment: Commitment) => {
  const args = connection._buildArgs([keys], commitment, "base64");
  const unsafeRes = await (connection as any)._rpcRequest("getMultipleAccounts", args);

  if (unsafeRes.error || !unsafeRes.result.value) {
    throw new Error(`failed to get account info: ${unsafeRes.error.message}`);
  }

  return {
    keys,
    values: unsafeRes.result.value as AccountInfo<string[]>[]
  };
};

function chunks<T>(keys: T[], size: number): T[][] {
  return Array.apply<number, T[], T[][]>(0, new Array(Math.ceil(keys.length / size)))
    .map((_, index) => keys.slice(index * size, (index + 1) * size));
}
