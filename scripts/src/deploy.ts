import HplMailbox from "./contracts/hpl_mailbox";
import HplValidatorAnnounce from "./contracts/hpl_validator_announce";
import HplHookAggregate from "./contracts/hpl_hook_aggregate";
import HplHookMerkle from "./contracts/hpl_hook_merkle";
import HplHookPausable from "./contracts/hpl_hook_pausable";
import HplHookRouting from "./contracts/hpl_hook_routing";
import HplHookRoutingCustom from "./contracts/hpl_hook_routing_custom";
import HplIgp from "./contracts/hpl_igp";
import HplIgpGasOracle from "./contracts/hpl_igp_oracle";
import HplIsmAggregate from "./contracts/hpl_ism_aggregate";
import HplIsmMultisig from "./contracts/hpl_ism_multisig";
import HplIsmRouting from "./contracts/hpl_ism_routing";
import HplTestMockHook from "./contracts/hpl_test_mock_hook";
import HplTestMockMsgReceiver from "./contracts/hpl_test_mock_msg_receiver";
import HplWarpCw20 from "./contracts/hpl_warp_cw20";
import HplWarpNative from "./contracts/hpl_warp_native";

import {
  Client,
  IsmType,
} from "../src/config";
import { addPad } from "./conv";

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

const name = (c: any) => c.contractName;

export const deploy_ism = async (
  client: Client,
  ism: IsmType,
  contracts: Contracts
): Promise<string> => {
  const { isms } = contracts;

  switch (ism.type) {
    case "multisig":
      console.log('Instantiate Multisig ISM contract')
      const multisig_ism_res = await isms.multisig.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
      });

      console.log('Enroll validators')
      console.log(ism)
      console.log(Object.entries(ism.validators).flatMap(([domain, validator]) =>
      validator.addrs.map((v) => ({
        domain: Number(domain),
        validator: v,
      }))
    ))
      await client.wasm.execute(
        client.signer,
        multisig_ism_res.address!,
        {
          enroll_validators: {
            set: Object.entries(ism.validators).flatMap(([domain, validator]) =>
              validator.addrs.map((v) => ({
                domain: Number(domain),
                validator: v,
              }))
            ),
          },
        },
        "auto"
      );

      console.log('Set thresholds')
      await client.wasm.execute(
        client.signer,
        multisig_ism_res.address!,
        {
          set_thresholds: {
            set: Object.entries(ism.validators).map(
              ([domain, { threshold }]) => ({
                domain: Number(domain),
                threshold,
              })
            ),
          },
        },
        "auto"
      );

      return multisig_ism_res.address!;

    case "aggregate":
      const aggregate_ism_res = await isms.aggregate.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
        isms: await Promise.all(
          ism.isms.map((v) => deploy_ism(client, v, contracts))
        ),
      });

      return aggregate_ism_res.address!;
    case "routing":
      const routing_ism_res = await isms.routing.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
      });

      await client.wasm.execute(
        client.signer,
        routing_ism_res.address!,
        {
          router: {
            set_routes: {
              set: await Promise.all(
                Object.entries(ism.isms).map(async ([domain, v]) => {
                  const route = await deploy_ism(client, v, contracts);
                  return { domain, route };
                })
              ),
            },
          },
        },
        "auto"
      );

      return routing_ism_res.address!;

    default:
      throw new Error("invalid ism type");
  }
};

