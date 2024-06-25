import { fromBech32 } from '@cosmjs/encoding';
import { IndexedTx, StargateClient } from '@cosmjs/stargate';
import { createHash } from 'crypto';
import * as fs from 'fs';
import * as readline from 'readline';
import { Readable } from 'stream';
import { finished } from 'stream/promises';

import { Client } from './config';

export const sleep = (ms: number) =>
  new Promise((resolve) => setTimeout(resolve, ms));

export function askQuestion(query: string) {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) =>
    rl.question(`${query} [Y/n] `, (ans) => {
      rl.close();
      resolve(ans.toLowerCase() == 'y' || ans == '' ? true : false);
    }),
  );
}

export const addPad = (v: string): string => {
  const s = v.startsWith('0x') ? v.slice(2) : v;
  return s.padStart(64, '0');
};

export const withLink = (text: string, url: string) =>
  `${text} (\u200B${url}\u200B)`;

export const extractByte32AddrFromBech32 = (addr: string): string => {
  const { data } = fromBech32(addr);
  const hexed = Buffer.from(data).toString('hex');
  return `0x${hexed.length === 64 ? hexed : addPad(hexed)}`;
};

export const downloadFile = async (url: string, dest: string) => {
  const res = await fetch(url);
  const fileStream = fs.createWriteStream(dest, { flags: 'wx' });
  await finished(Readable.fromWeb(res.body!).pipe(fileStream));
};

export const generateSha256 = (file: string): Promise<[string, string]> =>
  new Promise((resolve, reject) => {
    const stream = fs.createReadStream(file);
    const hash = createHash('sha256');

    stream.on('error', (err) => reject(err));
    stream.on('data', (chunk) => hash.update(chunk));
    stream.on('end', () => resolve([file, hash.digest('hex')]));
  });

export const waitTx = async (
  txHash: string,
  client: StargateClient,
  { waitMs, tryCount }: { waitMs: number; tryCount: number } = {
    waitMs: 1000,
    tryCount: 30,
  },
): Promise<IndexedTx> => {
  let found: IndexedTx | null = null;
  let count = 0;
  while (!found) {
    found = await client.getTx(txHash);
    count++;
    await sleep(waitMs); // default to 1s

    if (count > tryCount) {
      throw new Error(
        `max try count exceeded. count: ${tryCount}, waitMs: ${waitMs}`,
      );
    }
  }
  return found;
};

export function orSigner<SignerType extends string | '<signer>' = '<signer>'>(
  client: Client,
  v: SignerType,
): string {
  return v === '<signer>' ? client.signer : v;
}
