import { parseMappingData, parsePriceData, parseProductData } from "@pythnetwork/client";
import { Buffer } from "buffer";
import { AccountInfo, Commitment, Connection, PublicKey } from "@solana/web3.js";
import { RawData } from "../context";
import { cryptos, fetchCryptos, CryptoInfo } from "./unirest";

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
  ath?: number;
  ath_change_percentage?: number;
  max_supply?: number;
  circulating_supply?: number;
  confidence?: number;
  time?: number;
}

export async function init(): Promise<any> {
  conn = conn ?? new Connection(URL, "confirmed");
  if (!pending) {
    initCryptos();

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
        const products = productAccts.values.map((a) => parseProductData(Buffer.from(a.data)));
        console.log(`BRIDGE: parsed ${products.length} products, ${(performance.now() - start) | 0} msec`, products);
        const priceAccts = await getAccounts(
          conn,
          products.map((p) => p.priceAccountKey.toBase58()),
          "confirmed"
        );
        priceAccts.keys.forEach((key, i) => {
          const { product } = products[i];
          setPrice(product, priceAccts.values[i].data);
          conn.onAccountChange(new PublicKey(key), (acc) => setPrice(product, acc.data));
        });
        console.log(
          `BRIDGE: subscribed to ${priceAccts.keys.length} instruments, ${(performance.now() - start) | 0} msec`
        );
      }
    } catch (error) {
      console.error(error);
    }
  }
}

const getAccounts = async (connection: Connection, keys: string[], commitment: Commitment) => {
  const accounts = await Promise.all(chunks(keys, 99).map((chunk) => getAccountsCore(connection, chunk, commitment)));

  return {
    keys,
    values: accounts
      .map((a) =>
        a.values
          .filter((_) => _)
          .map((acc) => {
            const { data, ...rest } = acc;
            return {
              ...rest,
              data: Buffer.from(data[0], "base64")
            } as AccountInfo<Buffer>;
          })
      )
      .flat()
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
  return Array.apply<number, T[], T[][]>(0, new Array(Math.ceil(keys.length / size))).map((_, index) =>
    keys.slice(index * size, (index + 1) * size)
  );
}

async function initCryptos(): Promise<any> {
  await fetchCryptos();

  cryptos.forEach((item) => {
    const quote = createQuote(item);
    priceMap.set(quote.key, { ...quote, asset: "Crypto" });
  });
}

const setPrice = (product: any, buffer: Buffer) => {
  const data = parsePriceData(Buffer.from(buffer));
  const { price, confidence } = data;
  const { symbol, nasdaq_symbol, cms_symbol, asset_type: asset } = product;
  const key = nasdaq_symbol ?? cms_symbol ?? symbol;
  const delta = {
    key,
    symbol: key,
    price,
    confidence,
    time: Date.now()
  } as PythQuote;

  const obj = priceMap.get(delta.symbol);
  if (obj) {
    Object.assign(obj, delta);
  } else {
    const quote = { ...createQuote(cryptos.get(delta.symbol) || ({} as CryptoInfo)), ...delta, asset } as PythQuote;
    priceMap.set(quote.key, quote);
  }
};

function createQuote(crypto: CryptoInfo): PythQuote {
  const { market_cap_rank, market_data, key, description } = crypto;
  return {
    key,
    symbol: key,
    description: description?.en,
    market_cap_rank,
    market_cap: market_data?.market_cap?.usd,
    ath: market_data?.ath?.usd,
    ath_change_percentage: market_data?.ath_change_percentage?.usd,
    max_supply: market_data?.max_supply,
    circulating_supply: market_data?.circulating_supply
  } as PythQuote;
}
