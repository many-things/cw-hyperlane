import { Client, IsmType } from "../src/config";
import {
  HplMailbox,
  HplValidatorAnnounce,
  HplHookAggregate,
  HplHookMerkle,
  HplHookPausable,
  HplHookRouting,
  HplHookRoutingCustom,
  HplIgp,
  HplIsmAggregate,
  HplIsmMultisig,
  HplIsmRouting,
  HplTestMockHook,
  HplTestMockMsgReceiver,
  HplWarpCw20,
  HplWarpNative,
  HplIgpOracle,
  HplHookFee,
  HplTestMockIsm,
} from "./contracts";

export type Contracts = {
  core: {
    mailbox: HplMailbox;
    va: HplValidatorAnnounce;
  };
  hooks: {
    aggregate: HplHookAggregate;
    fee: HplHookFee;
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
  };
  mocks: {
    hook: HplTestMockHook;
    ism: HplTestMockIsm;
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

      console.log("Set thresholds");
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
