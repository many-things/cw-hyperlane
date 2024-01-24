import { Client, IsmType } from "../src/config";
import {
  HplHookAggregate,
  HplHookMerkle,
  HplHookPausable,
  HplHookRouting,
  HplHookRoutingCustom,
  HplIgp,
  HplIgpOracle,
  HplIsmAggregate,
  HplIsmMultisig,
  HplIsmPausable,
  HplIsmRouting,
  HplMailbox,
  HplTestMockHook,
  HplTestMockMsgReceiver,
  HplValidatorAnnounce,
  HplWarpCw20,
  HplWarpNative,
} from "./contracts";

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
    oracle: HplIgpOracle;
  };
  isms: {
    aggregate: HplIsmAggregate;
    multisig: HplIsmMultisig;
    routing: HplIsmRouting;
    pausable: HplIsmPausable;
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
      console.log("Instantiate Multisig ISM contract");
      const multisig_ism_res = await isms.multisig.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
      });

      console.log("Enroll validators");
      console.log(ism);
      console.log(
        Object.entries(ism.validators).flatMap(([domain, validator]) =>
          validator.addrs.map((v) => ({
            domain: Number(domain),
            validator: v,
          }))
        )
      );
      const setValidatorMessages = Object.entries(ism.validators).flatMap(([domain, set]) => ({
        contractAddress: multisig_ism_res.address!,
        msg: {
          set_validators: {
            domain: Number(domain),
            threshold: set.threshold,
            validators: set.addrs
          }
        }
      }));
      await client.wasm.executeMultiple(
        client.signer,
        setValidatorMessages,
        "auto"
      );

      return multisig_ism_res.address!;

    case "aggregate":
      let sub_modules = [];
      for (const v of ism.isms) {
        sub_modules.push(await deploy_ism(client, v, contracts));
      }
      const aggregate_ism_res = await isms.aggregate.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
        isms: sub_modules,
        threshold: ism.threshold,
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

    case "pausable":

      const pausable_ism_res = await isms.pausable.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
        paused: ism.paused ?? false
      });

      return pausable_ism_res.address!;

    default:
      throw new Error(`unsupported ism ${ism}`);
  }
};
