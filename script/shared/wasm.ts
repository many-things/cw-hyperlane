import fs from 'fs';
import path from 'path';

import { Config } from './config';
import { defaultArtifactPath } from './constants';
import { generateSha256 } from './utils';

function getWasmFilesPath(
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  },
): string[] {
  try {
    const files = fs.readdirSync(artifactPath);
    return files
      .filter((file) => file.endsWith('.wasm'))
      .map((file) => path.join(artifactPath, file));
  } catch (err) {
    console.error(
      '[error]'.red,
      'cannot find wasm folder.',
      'did you compiled the wasm projects?',
    );
    process.exit(1);
  }
}

export async function loadWasmFileDigest(
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  },
): Promise<Record<string, string>> {
  return Object.fromEntries(
    await Promise.all(getWasmFilesPath({ artifactPath }).map(generateSha256)),
  );
}

export function getWasmPath(
  contractName: string,
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  },
): string {
  return path.join(artifactPath, `${contractName}.wasm`);
}

export type ContractInfoResp = {
  address: string;
  contract_info: {
    code_id: string;
    creator: string;
    admin?: string;
    label: string;
    created: {
      block_height: string;
      tx_index: string;
    };
    ibc_por_id?: string;
    extension?: any;
  };
};

export async function getContractInfo(
  network: Config['networks'][number],
  addr: string,
): Promise<ContractInfoResp | undefined> {
  try {
    const res = await fetch(
      path.join(network.endpoint.rest, '/cosmwasm/wasm/v1/contract/', addr),
    );
    const body = await res.json();

    return body as ContractInfoResp;
  } catch (err) {
    console.error('Error fetching contract info', err);
    return undefined;
  }
}
