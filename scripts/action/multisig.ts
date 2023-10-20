import { Command } from "commander";

import { version } from "../package.json";
import { loadContext } from "../src/load_context";
import { config } from "../src/config";
import { fromBech32 } from "@cosmjs/encoding";
import { Secp256k1, keccak256 } from "@cosmjs/crypto";
import { readFileSync, writeFileSync } from "fs";
import { DirectSecp256k1Wallet } from "@cosmjs/proto-signing";

type CheckpointInfo = {
  origin_domain: number;
  origin_merkle_tree: string;
  merkle_root: string;
  merkle_index: number;
};

const toHex = (v: Uint8Array): string => {
  return Buffer.from(v).toString("hex");
};

const fromHex = (v: string): Uint8Array => {
  return Buffer.from(v, "hex");
};

const u8 = (v: string): Uint8Array => {
  return fromHex(Number(v).toString(16).padStart(8, "0"));
};

const program = new Command();
const common = { output: `${process.cwd()}/signature.json` };

program.name("Multisig CLI").version(version);

program
  .command("sign")
  .argument("<origin_domain>", 'origin domain, e.g. "5"')
  .argument("<merkle_root>", "merkle root in hex")
  .argument("<merkle_index>", "merkle index")
  .argument("<message_id>", "message id in hex")
  .option("-o, --output <file>", "output file", common.output)
  .option("-p --prefix <prefix>", "bech32 prefix", "dual")
  .requiredOption("-k --key <private_key>", "private key")
  .action(sign);

program
  .command("join")
  .argument("<signatures...>", "signature files (comma separated)")
  .option("-o, --output <file>", "output file", common.output)
  .option("-p --prefix <prefix>", "bech32 prefix", "dual")
  .action(join);

program.parseAsync(process.argv).catch(console.error);

async function sign(
  origin_domain_str: string,
  merkle_root: string,
  merkle_index_str: string,
  message_id: string,
  options: { output: string; prefix: string; key: string }
) {
  const ctx = loadContext(config.network.id);
  const origin_domain = u8(origin_domain_str);
  const merkle_index = u8(merkle_index_str);

  const origin_merkle_str = ctx.contracts.hpl_hook_merkle.address!;
  const origin_merkle = Buffer.from(fromBech32(origin_merkle_str).data);

  const domain_hash = keccak256(
    Buffer.concat([
      origin_domain,
      origin_merkle,
      Buffer.from("HYPERLANE", "utf-8"),
    ])
  );

  const multisig_hash = keccak256(
    Buffer.concat([
      domain_hash,
      fromHex(merkle_root),
      merkle_index,
      fromHex(message_id),
    ])
  );

  const verify_digest = keccak256(
    Buffer.concat([
      Buffer.from(`\x19Ethereum Signed Message:\n${multisig_hash.length}`),
      multisig_hash,
    ])
  );

  const keypair = await Secp256k1.makeKeypair(fromHex(options.key));
  const key = await DirectSecp256k1Wallet.fromKey(
    keypair.privkey,
    options.prefix
  );

  const [{ address }] = await key.getAccounts();

  const signature = await Secp256k1.createSignature(
    verify_digest,
    keypair.privkey
  );

  type Output = {
    address: string;
    signature: string;
  } & CheckpointInfo;

  const output: Output = {
    origin_domain: Number(origin_domain_str),
    origin_merkle_tree: origin_merkle.toString("hex"),
    merkle_root,
    merkle_index: Number(merkle_index_str),
    address: address,
    signature: toHex(signature.toFixedLength()),
  };

  writeFileSync(options.output, JSON.stringify(output, null, 2));
}

async function join(
  signature_paths: string[],
  options: { output: string; prefix: string }
) {
  const ctx = loadContext(config.network.id);

  const origin_merkle_str = ctx.contracts.hpl_hook_merkle.address!;
  const origin_merkle = Buffer.from(fromBech32(origin_merkle_str).data);

  type Output = {
    address: string;
    signature: string;
  } & CheckpointInfo;

  type Joined = {
    signatures: Record<string, string>;
  } & CheckpointInfo;

  let joined: Joined | null = null;

  for (const path of signature_paths) {
    const output: Output = JSON.parse(readFileSync(path, "utf-8"));

    if (joined) {
      joined.signatures[output.address!] = output.signature!;
      continue;
    }

    joined = {
      origin_domain: output.origin_domain,
      origin_merkle_tree: output.origin_merkle_tree,
      merkle_root: output.merkle_root,
      merkle_index: output.merkle_index,
      signatures: {
        [output.address]: output.signature,
      },
    };
  }

  if (!joined) {
    console.error("no signature given");
    return;
  }

  const metadata = Buffer.concat([
    fromHex(joined.origin_merkle_tree),
    fromHex(joined.merkle_root),
    u8(joined.merkle_index.toString()),
    Buffer.concat(Object.values(joined.signatures).map((v) => fromHex(v))),
  ]);

  console.log(metadata.toString("hex"));
}
