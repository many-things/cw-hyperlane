import { Client, IsmType } from '../shared/config';
import { Context, ContextIsm } from '../shared/context';
import { deployContract, executeMultiMsg } from '../shared/contract';

const deployRoutingIsm = async (
  ctx: Context,
  client: Client,
  ism: Extract<IsmType, { type: 'routing' }>,
) => {
  const routes = [];
  for (const [domain, v] of Object.entries(ism.isms)) {
    routes.push({
      domain: parseInt(domain),
      route: await deployIsm(ctx, client, v),
    });
  }

  const routing = await deployContract(ctx, client, 'hpl_ism_routing', {
    owner: ism.owner === '<signer>' ? client.signer : ism.owner,
    isms: routes.map(({ domain, route }) => ({
      domain,
      address: route.address,
    })),
  });

  return {
    ...routing,
    isms: routes.reduce((acc, v) => ({ [v.domain]: v.route, ...acc })),
  };
};

export async function deployIsm(
  ctx: Context,
  client: Client,
  ism: Exclude<IsmType, number[]>,
): Promise<ContextIsm> {
  switch (ism.type) {
    // deploy multisig ism
    case 'multisig': {
      const multisig = await deployContract(ctx, client, 'hpl_ism_multisig', {
        owner: ism.owner === '<signer>' ? client.injective_signer : ism.owner,
      });

      if (ism.validators != undefined) {
        await executeMultiMsg(
          client,
          Object.entries(ism.validators).map(
            ([domain, { addrs, threshold }]) => ({
              contract: multisig,
              msg: {
                set_validators: {
                  domain: Number(domain),
                  threshold,
                  validators: addrs.map((v) =>
                    v === '<signer>' ? client.signer_addr : v,
                  ),
                },
              },
            }),
          ),
        );
      }

      return multisig;
    }

    // deploy aggregate ism
    case 'aggregate': {
      const aggr = [];
      for (const v of ism.isms) {
        aggr.push(await deployIsm(ctx, client, v));
      }

      const aggregate = await deployContract(ctx, client, 'hpl_ism_aggregate', {
        owner: ism.owner === '<signer>' ? client.signer : ism.owner,
        isms: aggr.map((v) => v.address),
      });

      return { ...aggregate, isms: aggr };
    }

    // deploy routing ism
    case 'routing': {
      return deployRoutingIsm(ctx, client, ism);
    }

    default: {
      throw new Error('invalid ism type');
    }
  }
}
