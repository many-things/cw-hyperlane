import colors from "colors";
import { loadWasmFileDigest, getWasmPath } from "./load_wasm";
import { loadContext, saveContext } from "./load_context";
import { getTargetContract, getTargetContractName } from "./contracts";
import { CodeUpdate, CodeCreate, Context } from "./types";
import * as readline from 'readline';
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import { AxiosError } from "axios";
import { CONTAINER } from "./ioc";
import { runMigrations } from "./migrations";

colors.enable();
const NETWORK = "osmo-test-5";

function askQuestion(query: string) {
  const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
  });

  return new Promise(resolve => rl.question(`${query} [Y/n] `, ans => {
      rl.close();
      resolve(ans.toLowerCase() == 'y' ? true : false);
  }))
}

async function getSigningClient(): Promise<{ client: SigningCosmWasmClient, address: string }> {
  const mnemonic = process.env['SIGNING_MNEMONIC'] as string;
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {prefix: 'osmo'});
  const [{address}] = await wallet.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner('https://rpc.osmotest5.osmosis.zone/', wallet, {
    gasPrice: GasPrice.fromString('0.025uosmo'),
  });
  return { client, address };
}

async function main() {
  const digest = await loadWasmFileDigest();
  const context = await loadContext(NETWORK);
  const targetContractName = getTargetContractName();

  const {client, address} = await getSigningClient();
  context.address = address;

  CONTAINER.bind(Context).toConstantValue(context);
  const contracts = getTargetContract(context, client, address, CONTAINER);
  console.log("check exist contracts....");

  const codeChanges = targetContractName.map((contractName) => {
    const ctxContract = context.contracts[contractName];
    const currentDigest = digest[getWasmPath(contractName)];
    if (ctxContract === undefined) {
      return {
        contractName,
        digest: currentDigest,
      } as CodeCreate;
    } else if (ctxContract.digest != currentDigest) {
      return {
        contractName,
        codeId: ctxContract.codeId,
        digest: currentDigest,
      } as CodeUpdate;
    }
  }).filter(v => v !== undefined);

  if (codeChanges.length !== 0) {
    console.log(`Found ${codeChanges.length} contracts to upload.\n`);
    let creationExists = false;
    codeChanges.forEach((v) => {
      if (v === undefined) return;

      if ('codeId' in v) {
        console.log("UPDATE".yellow, `${v.contractName} (${v.codeId})`.padEnd(30), '|', v.digest);
      } else {
        console.log("CREATE".green, `${v.contractName}`.padEnd(30), '|', v.digest);
        creationExists = true;
      }
    });

    // check upload
    const askUpload = await askQuestion("Do you want to upload contracts?");
    if (!askUpload && creationExists) {
      console.log("\n[ERROR] You must upload all new contracts.".red);
      process.exit(1);
    } else if (askUpload) {
      console.log("\nuploading...\n");

      for(let v of codeChanges) {
        if (v === undefined) return;

        const contract = contracts[v.contractName];
        if (contract === undefined) return;

        process.stdout.write("[UPLOAD]".gray);
        process.stdout.write(` ${v.contractName} ... `);

        try{
          contract.digest = v.digest;
          const contractContext = await contract.uplaod();
          context.contracts[v.contractName] = contractContext;
          saveContext("osmo-test-5", context);

          console.log("OK".green, "as", contractContext.codeId);
        } catch(e) {
          const err = e as AxiosError;
          console.log("FAILED".red, "=>", err);
        }
      };
    }
  } else {
    console.log("No contracts to upload.");
  }

  runMigrations(NETWORK, false);
}

main();
