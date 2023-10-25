import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Context } from "../src/types";
import { Client } from "../src/config";
import HplMailbox from "../src/contracts/hpl_mailbox";
import HplValidatorAnnounce from "../src/contracts/hpl_validator_announce";
import HplHookAggregate from "../src/contracts/hpl_hook_aggregate";
import HplHookMerkle from "../src/contracts/hpl_hook_merkle";
import HplHookPausable from "../src/contracts/hpl_hook_pausable";
import HplHookRouting from "../src/contracts/hpl_hook_routing";
import HplHookRoutingCustom from "../src/contracts/hpl_hook_routing_custom";
import HplIgp from "../src/contracts/hpl_igp";
import HplIgpGasOracle from "../src/contracts/hpl_igp_oracle";
import HplIsmAggregate from "../src/contracts/hpl_ism_aggregate";
import HplIsmMultisig from "../src/contracts/hpl_ism_multisig";
import HplIsmRouting from "../src/contracts/hpl_ism_routing";
import HplTestMockHook from "../src/contracts/hpl_test_mock_hook";
import HplTestMockMsgReceiver from "../src/contracts/hpl_test_mock_msg_receiver";
import HplWarpCw20 from "../src/contracts/hpl_warp_cw20";
import HplWarpNative from "../src/contracts/hpl_warp_native";

type Const<T> = new (
  address: string | undefined,
  codeId: number | undefined,
  digest: string,
  signer: string,
  client: SigningCosmWasmClient
) => T;

export type Contracts = {
  core: {
    mailbox: HplMailbox;
    va: HplValidatorAnnounce;
  };
  hooks: {
    aggregate: HplHookAggregate;
    merkle: HplHookMerkle;
    pausable: HplHookPausable;
    routing: HplHookRouting;
    routing_custom: HplHookRoutingCustom;
    routing_fallback: HplHookRoutingCustom;
  };
  igp: {
    core: HplIgp;
    oracle: HplIgpGasOracle;
  };
  isms: {
    aggregate: HplIsmAggregate;
    multisig: HplIsmMultisig;
    routing: HplIsmRouting;
  };
  mocks: {
    hook: HplTestMockHook;
    receiver: HplTestMockMsgReceiver;
  };
  warp: {
    cw20: HplWarpCw20;
    native: HplWarpNative;
  };
};

export class ContractFetcher {
  constructor(private ctx: Context, private client: Client) {}

  public get<T>(f: Const<T>, name: string): T {
    return new f(
      this.ctx.contracts[name].address,
      this.ctx.contracts[name].codeId,
      this.ctx.contracts[name].digest,
      this.client.signer,
      this.client.wasm
    );
  }

  public getContracts(): Contracts {
    return {
      core: {
        mailbox: this.get(HplMailbox, "hpl_mailbox"),
        va: this.get(HplValidatorAnnounce, "hpl_validator_announce"),
      },
      hooks: {
        aggregate: this.get(HplHookAggregate, "hpl_hook_aggregate"),
        merkle: this.get(HplHookMerkle, "hpl_hook_merkle"),
        pausable: this.get(HplHookPausable, "hpl_hook_pausable"),
        routing: this.get(HplHookRouting, "hpl_hook_routing"),
        routing_custom: this.get(
          HplHookRoutingCustom,
          "hpl_hook_routing_custom"
        ),
        routing_fallback: this.get(
          HplHookRoutingCustom,
          "hpl_hook_routing_fallback"
        ),
      },
      igp: {
        core: this.get(HplIgp, "hpl_igp"),
        oracle: this.get(HplIgpGasOracle, "hpl_igp_oracle"),
      },
      isms: {
        aggregate: this.get(HplIsmAggregate, "hpl_ism_aggregate"),
        multisig: this.get(HplIsmMultisig, "hpl_ism_multisig"),
        routing: this.get(HplIsmRouting, "hpl_ism_routing"),
      },
      mocks: {
        hook: this.get(HplTestMockHook, "hpl_test_mock_hook"),
        receiver: this.get(
          HplTestMockMsgReceiver,
          "hpl_test_mock_msg_receiver"
        ),
      },
      warp: {
        cw20: this.get(HplWarpCw20, "hpl_warp_cw20"),
        native: this.get(HplWarpNative, "hpl_warp_native"),
      },
    };
  }
}
