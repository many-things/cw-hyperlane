import {
  PublicClient,
  Transport,
  Chain,
  Account,
  Address,
  getContractAddress,
} from "viem";

export const addPad = (v: string): string => {
  const s = v.startsWith("0x") ? v.slice(2) : v;
  return s.padStart(64, "0");
};

export const logTx = (title: string, tx: string) =>
  console.log(
    "=>".grey,
    title + "\n",
    "=>".green,
    `tx hash: "${tx.green}". Waiting for confirmation...`
  );

export const expectNextContractAddr = async (
  query: PublicClient<Transport, Chain>,
  account: Account
): Promise<Address> => {
  const nonce = await query.getTransactionCount(account);

  const next = getContractAddress({
    opcode: "CREATE",
    from: account.address,
    nonce: BigInt(nonce),
  });

  return next;
};
