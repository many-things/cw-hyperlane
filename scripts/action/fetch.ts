import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Context } from "../src/types";
import { Client } from "../src/config";
import { Contracts } from "../src/deploy";
import {
  HplMailbox,
  HplValidatorAnnounce,
  HplHookAggregate,
  HplHookMerkle,
  HplHookPausable,
  HplHookRouting,
  HplHookRoutingCustom,
  HplIgp,
  HplIgpOracle,
  HplIsmAggregate,
  HplIsmMultisig,
  HplIsmRouting,
  HplTestMockHook,
  HplTestMockMsgReceiver,
  HplWarpCw20,
  HplWarpNative,
  HplHookFee,
  HplTestMockIsm,
} from "../src/contracts";

type Const<T> = new (
  address: string | undefined,
  codeId: number | undefined,
  digest: string,
  signer: string,
  client: SigningCosmWasmClient
) => T;

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
        fee: this.get(HplHookFee, "hpl_hook_fee"),
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
        oracle: this.get(HplIgpOracle, "hpl_igp_oracle"),
      },
      isms: {
        aggregate: this.get(HplIsmAggregate, "hpl_ism_aggregate"),
        multisig: this.get(HplIsmMultisig, "hpl_ism_multisig"),
        routing: this.get(HplIsmRouting, "hpl_ism_routing"),
      },
      mocks: {
        hook: this.get(HplTestMockHook, "hpl_test_mock_hook"),
        ism: this.get(HplTestMockIsm, "hpl_test_mock_ism"),
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
