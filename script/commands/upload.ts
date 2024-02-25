/**
 * Upload command for contract codes
 * 1. local
 *  - upload local artifacts
 * 2. remote
 *  - upload remote artifacts
 * 3. remote-list
 *  - list available releases from github (check `../common/github.ts` to see how it works)
 */

import * as fs from "fs";
import { Command } from "commander";
import { CodeDetails, IndexedTx } from "@cosmjs/cosmwasm-stargate";

import { getWasmPath, loadWasmFileDigest } from "../shared/wasm";
import { CONTAINER, Dependencies } from "../shared/ioc";
import {
  MIN_RELEASE_VERSION,
  downloadReleases,
  getReleases,
} from "../shared/github";
import {
  contractNames,
  defaultArtifactPath,
  defaultTmpDir,
} from "../shared/constants";
import { askQuestion, sleep, waitTx } from "../shared/utils";
import { saveContext } from "../shared/context";
import { ContractNames } from "../shared/contract";

// ============ Command Definitions

const uploadCmd = new Command("upload")
  .description("Upload contract codes")
  .option("-c --contracts <contracts...>", "specify contracts to upload")
  .configureHelp({ showGlobalOptions: true });

uploadCmd
  .command("local")
  .description("upload artifacts from local")
  .option("-a --artifacts <artifacts dir>", "artifacts", defaultArtifactPath)
  .action(async (_, cmd) => upload(cmd.optsWithGlobals()));

uploadCmd
  .command("remote")
  .description("upload artifacts from remote")
  .argument("<tag-name>", `name of release tag. min: ${MIN_RELEASE_VERSION}`)
  .option("-o --out <out dir>", "artifact output directory", defaultTmpDir)
  .action(handleRemote);

uploadCmd
  .command("remote-list")
  .description("list all available public release of cw-hyperlane")
  .action(handleRemoteList);

export { uploadCmd };

// ============ Handler Functions

async function handleRemote(tagName: string, _: any, cmd: any): Promise<void> {
  const opts = cmd.optsWithGlobals();

  if (tagName < MIN_RELEASE_VERSION)
    throw new Error(`${tagName} < ${MIN_RELEASE_VERSION}`);

  const releases = await getReleases();
  if (!releases[tagName])
    throw new Error(
      `release ${tagName} not found in remote.` +
        "try 'upload remote-list' to see available releases."
    );

  // make directory if not exists
  if (!fs.existsSync(opts.out)) fs.mkdirSync(opts.out, { recursive: true });

  const artifactPath = await downloadReleases(releases[tagName], opts.out);

  console.log("Downloaded artifacts to", artifactPath.green);

  return upload({ ...opts, artifacts: artifactPath });
}

async function handleRemoteList() {
  const releases = await getReleases();

  console.log("Available releases:".green);
  for (const [tagName, codes] of Object.entries(releases)) {
    console.log("-", `[${tagName}]`.blue);
    console.log("ㄴ codes:".grey, `(${codes})`);
  }
}

// ============ Business Logic

type UploadArgs = {
  artifacts: string;
  contracts?: ContractNames[];
  networkId: string;
};

async function upload({
  artifacts: artifactPath,
  contracts: uploadTargets,
  networkId,
}: UploadArgs) {
  (uploadTargets || []).forEach((v) => {
    if (!contractNames.includes(v))
      throw new Error(
        `invalid contract name ${v}.` +
          "try 'contract list' to see available contracts."
      );
  });

  const digest = await loadWasmFileDigest({ artifactPath });
  const { ctx, client }: Dependencies = CONTAINER.get(Dependencies);

  // query code details of context artifacts
  const codeIds = Object.fromEntries(
    await Promise.all(
      (Object.values(contractNames) as ContractNames[])
        .filter((k) => (uploadTargets ? uploadTargets.includes(k) : true))
        .map(async (k) => [
          k,
          ctx.artifacts[k] &&
            (await client.wasm.getCodeDetails(ctx.artifacts[k])),
        ])
    )
  ) as Record<ContractNames, CodeDetails | undefined>;

  // checking code changes
  console.log("Checking code changes...".green);

  const listDiff = Object.entries(codeIds)
    .map(([v, codeId]) => {
      const oldCodeChecksum = codeId?.checksum;
      const newCodeChecksum = digest[getWasmPath(v, { artifactPath })];

      if (oldCodeChecksum && oldCodeChecksum === newCodeChecksum) {
        console.log("[NO-CHANGE]".green.padEnd(12, " "), v.padEnd(30, " "));
        return undefined;
      }

      if (!oldCodeChecksum) {
        console.log(
          "[NEW]".yellow.padEnd(12, " "),
          v.padEnd(30, " "),
          newCodeChecksum
        );
      } else {
        console.log(
          "[REPLACE]".yellow.padEnd(12, " "),
          v.padEnd(30, " "),
          oldCodeChecksum,
          "!=",
          newCodeChecksum
        );
      }

      return v;
    })
    .filter((v) => v !== undefined) as ContractNames[];

  if (listDiff.length === 0) {
    console.log("No changes detected.");
    return;
  }

  if (!(await askQuestion("Do you want to proceed? (y/n)"))) {
    console.log("Aborted.");
    return;
  }
  console.log("Proceeding to upload...");

  let okCount = 0;
  for (const diff of listDiff) {
    const upload = await client.wasm.upload(
      client.signer,
      fs.readFileSync(getWasmPath(diff, { artifactPath })),
      "auto"
    );

    const receipt = await waitTx(upload.transactionHash, client.stargate);

    if (receipt.code > 0) {
      console.error(
        "[FAILURE]".red.padEnd(10, " "),
        `${diff.padEnd(30, " ")}`,
        `tx: ${upload.transactionHash}`
      );
      continue;
    }

    console.log(
      "[SUCCESS]".green.padEnd(10, " "),
      `${diff.padEnd(30, " ")}`,
      `codeId: ${upload.codeId}, tx: ${upload.transactionHash}`
    );

    ctx.artifacts[diff] = upload.codeId;
    okCount++;
  }

  if (okCount === 0) {
    console.error(
      "[FAILURE]".red.padEnd(10, " "),
      "every uploads have failed."
    );
    return;
  }

  console.log(
    "[SUCCESS]".green.padEnd(10, " "),
    `uploaded ${okCount} contracts.`
  );
  saveContext(networkId, ctx);
}
