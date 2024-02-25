import { Client, IgpHookType, getNetwork } from "../shared/config";
import { Context, ContextHook } from "../shared/context";
import { deployContract, executeContract } from "../shared/contract";

export const deployIgp = async (
  networkId: string,
  ctx: Context,
  client: Client,
  igpType: IgpHookType
): Promise<ContextHook> => {
  const { hrp, gas } = getNetwork(networkId);

  // init igp
  const igp = await deployContract(ctx, client, "hpl_igp", {
    hrp,
    owner: client.signer,
    gas_token: igpType.token || gas.denom,
    beneficiary: client.signer,
    default_gas_usage: igpType.default_gas_usage,
  });

  // init igp oracle

  const igpOracle = await deployContract(ctx, client, "hpl_igp_oracle", {
    owner: client.signer,
  });

  await executeContract(client, igpOracle, {
    set_remote_gas_data_configs: {
      configs: Object.entries(igpType.configs).map(([domain, v]) => ({
        remote_domain: Number(domain),
        token_exchange_rate: v.exchange_rate.toString(),
        gas_price: v.gas_price.toString(),
      })),
    },
  });

  await executeContract(client, igp, {
    router: {
      set_routes: {
        set: Object.keys(igpType.configs).map((domain) => ({
          domain: Number(domain),
          route: igpOracle.address,
        })),
      },
    },
  });

  return { ...igp, oracle: igpOracle };
};
