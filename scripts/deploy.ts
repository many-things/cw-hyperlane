import {
  ExecuteResult,
  SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";

import { loadContext } from "./src/load_context";
import HplMailbox from "./src/contracts/hpl_mailbox";
import { BaseContract, Context } from "./src/types";
import HplHookMerkle from "./src/contracts/hpl_hook_merkle";
import HplTestMockHook from "./src/contracts/hpl_test_mock_hook";
import HplIgpGasOracle from "./src/contracts/hpl_igp_oracle";
import HplIgp from "./src/contracts/hpl_igp";
import HplIsmMultisig from "./src/contracts/hpl_ism_multisig";
import { writeFileSync } from "fs";
import HplValidatorAnnounce from "./src/contracts/hpl_validator_announce";
import HplTestMockMsgReceiver from "./src/contracts/hpl_test_mock_msg_receiver";

const NETWORK_ID = process.env.NETWORK_ID || "osmo-test-5";
const NETWORK_HRP = process.env.NETWORK_HRP || "osmo";
const NETWORK_URL =
  process.env.NETWORK_URL || "https://rpc.osmotest5.osmosis.zone";
const NETWORK_GAS = process.env.NETWORK_GAS || "0.025uosmo";

async function getSigningClient(): Promise<{
  client: SigningCosmWasmClient;
  address: string;
}> {
  const mnemonic = process.env["SIGNING_MNEMONIC"] as string;
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: NETWORK_HRP,
  });
  const [{ address }] = await wallet.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner(
    NETWORK_URL,
    wallet,
    {
      gasPrice: GasPrice.fromString(NETWORK_GAS),
    }
  );
  return { client, address };
}

type Const<T> = new (
  address: string | undefined,
  codeId: number | undefined,
  digest: string,
  signer: string,
  client: SigningCosmWasmClient
) => T;

class ContractFetcher {
  constructor(
    private ctx: Context,
    private owner: string,
    private client: SigningCosmWasmClient
  ) {}

  public get<T>(f: Const<T>, name: string): T {
    return new f(
      this.ctx.contracts[name].address,
      this.ctx.contracts[name].codeId,
      this.ctx.contracts[name].digest,
      this.owner,
      this.client
    );
  }
}

async function main() {
  const { client, address: owner } = await getSigningClient();

  const ctx = loadContext(NETWORK_ID);

  const fetcher = new ContractFetcher(ctx, owner, client);

  const mailbox = fetcher.get(HplMailbox, "hpl_mailbox");
  const va = fetcher.get(HplValidatorAnnounce, "hpl_validator_announce");

  const hook_merkle = fetcher.get(HplHookMerkle, "hpl_hook_merkle");
  const igp_oracle = fetcher.get(HplIgpGasOracle, "hpl_igp_oracle");
  const igp = fetcher.get(HplIgp, "hpl_igp");
  const ism_multisig = fetcher.get(HplIsmMultisig, "hpl_ism_multisig");

  const test_mock_hook = fetcher.get(HplTestMockHook, "hpl_test_mock_hook");
  const test_mock_receiver = fetcher.get(
    HplTestMockMsgReceiver,
    "hpl_test_mock_msg_receiver"
  );

  // init mailbox
  ctx.contracts[mailbox.contractName] = await mailbox.instantiate({
    hrp: "dual",
    owner,
    domain: 33333,
  });

  // init validator announce
  ctx.contracts[va.contractName] = await va.instantiate({
    hrp: "dual",
    mailbox: ctx.contracts[mailbox.contractName].address,
  });

  // init merkle hook - (required hook)
  ctx.contracts[hook_merkle.contractName] = await hook_merkle.instantiate({
    owner: ctx.address!,
    mailbox: ctx.contracts[mailbox.contractName].address,
  });

  // init mock hook - (default hook)
  ctx.contracts[test_mock_hook.contractName] = await test_mock_hook.instantiate(
    {}
  );

  // init igp oracle
  ctx.contracts[igp_oracle.contractName] = await igp_oracle.instantiate({
    owner: ctx.address!,
  });

  // init igp
  ctx.contracts[igp.contractName] = await igp.instantiate({
    hrp: "dual",
    owner: ctx.address!,
    mailbox: ctx.contracts[mailbox.contractName].address,
    gas_token: "token",
    beneficiary: ctx.address!,
  });


  // init ism multisig
  ctx.contracts[ism_multisig.contractName] = await ism_multisig.instantiate({
    hrp: "dual",
    owner: ctx.address!,
  });

  // init test mock msg receiver
  ctx.contracts[test_mock_receiver.contractName] =
    await test_mock_receiver.instantiate({ hrp: "dual" });

  // pre-setup
  await client.executeMultiple(
    owner,
    [
      {
        contractAddress: ctx.contracts[mailbox.contractName].address!,
        msg: {
          set_default_ism: {
            ism: ctx.contracts[ism_multisig.contractName].address!,
          },
        },
      },
      {
        contractAddress: ctx.contracts[mailbox.contractName].address!,
        msg: {
          set_default_hook: {
            hook: ctx.contracts[test_mock_hook.contractName].address!,
          },
        },
      },
      {
        contractAddress: ctx.contracts[mailbox.contractName].address!,
        msg: {
          set_required_hook: {
            hook: ctx.contracts[hook_merkle.contractName].address!,
          },
        },
      },
    ],
    "auto"
  );

  writeFileSync("./save.json", JSON.stringify(ctx, null, 2));
}

main().catch(console.error);
