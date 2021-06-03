// eslint-disable-next-line no-undef
var unirest = require("unirest");

export interface CryptoInfo {
  id?: string;
  symbol?: string;
  name?: string;
  description?: { en: string };
  image?: { thumb?: string; small?: string; large?: string };
  market_cap_rank?: number;
  market_data?: {
    market_cap?: { usd?: number };
    sparkline_7d: { price: number[] };
  };
}

// there is a mismatch between pyth-based feed crypto ids and Coinbase database so we'll just preload a sample set instead
const cryptoMap = {
  "BCH/USD": "bitcoin-cash",
  "BNB/USD": "binancecoin",
  "BTC/USD": "bitcoin",
  "DOGE/USD": "dogecoin",
  "ETH/USD": "ethereum",
  "LTC/USD": "litecoin",
  "LUNA/USD": "terra-luna",
  "SOL/USD": "solana",
  "SRM/USD": "serum",
  "USDC/USD": "usd-coin",
  "USDT/USD": "tether",
};
export const cryptos = new Map<string, CryptoInfo>();

export async function getCrypto(id: string = "bitcoin"): Promise<CryptoInfo> {
  return new Promise((resolve, reject) => {
    let req = unirest("GET", `https://coingecko.p.rapidapi.com/coins/${id}`);
    //var req = unirest("GET", "https://coingecko.p.rapidapi.com/coins/list");

    req.query({
      localization: false,
      tickers: false,
      market_data: true,
      community_data: false,
      developer_data: false,
      sparkline: true,
    });

    req.headers({
      "x-rapidapi-key": "45d3d92d55msh910a04134367825p10bf61jsn665ea232afc4",
      "x-rapidapi-host": "coingecko.p.rapidapi.com",
      useQueryString: true,
    });

    req.end(function (res: any) {
      if (res.error) {
        reject(res.error);
      } else {
        const crypto = res.body as CryptoInfo;
        resolve(crypto);
      }
    });
  });
}

export async function getCryptos(ids?: string[]): Promise<CryptoInfo[]> {
  ids = ids ?? Object.values(cryptoMap);
  const promises = ids.map(getCrypto);
  const items = await Promise.all(promises);
  items.map((o) => cryptos.set(o.symbol?.toUpperCase() + "/USD", o));
  console.log(`CoinGecko: loaded ${items.length} cryptos`);
  return items;
}
