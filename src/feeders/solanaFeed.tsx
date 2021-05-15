import { Connection } from "@solana/web3.js";
import { fill } from "../context";

let buffer: ArrayBuffer;
let raw_data: Block[] = [];
let conn: Connection;

const MAX_ROW_COUNT = 50;
const URL = "https://api.mainnet-beta.solana.com";

interface Block {
  tranCount: number;
  slot: number;
  time: number;
}

export async function generateBlockData(
  data_width: number
): Promise<Float64Array> {
  const item = (slot: number, tranCount: number = 0) =>
    ({
      tranCount,
      slot,
      time: Date.now(),
    } as Block);

  const setTranCount = (item: Block, count: number = 0) => {
    item.tranCount = count;
    item.time = Date.now();
  };
  buffer = buffer ?? new ArrayBuffer(MAX_ROW_COUNT * data_width * 8);

  conn = conn ?? new Connection(URL, "confirmed");
  const epoch = await conn.getEpochInfo();
  console.log(`EPOCH`, epoch);

  if (raw_data[0]?.slot === epoch.absoluteSlot) {
    //const block = await conn.getConfirmedBlock(epoch.absoluteSlot);
    setTranCount(raw_data[0], epoch.transactionCount);
  } else {
    // if (raw_data.length) {
    //   const block = await conn.getConfirmedBlock(raw_data[0].slot);
    //   setTranCount(raw_data[0], block.transactions.length);
    // }
    raw_data.unshift(item(epoch.absoluteSlot, epoch.transactionCount));
  }

  raw_data = raw_data.slice(0, MAX_ROW_COUNT);

  return fill<Block>(
    buffer,
    raw_data,
    data_width,
    (data: Block, col: number) => {
      switch (col) {
        case 0:
          return data.slot;
        case 1:
          return data.tranCount;
        case 2:
          return data.time;
        default:
          return 0;
      }
    }
  );
}
