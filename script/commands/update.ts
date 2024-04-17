import { Command } from 'commander';

import { IgpHookType, config } from '../shared/config';
import { typed, ContextHook } from '../shared/context';
import { executeMultiMsg } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';

export const updateCmd = new Command('update')
  .description('Register new chain to existing contracts')
  .configureHelp({ showGlobalOptions: true })
;

updateCmd.command('igp-oracle').action(handleRegisterIgpOracle);
updateCmd.command('ism-multisig').action(handleRegisterIsm);

type AggregateHook = typed<'hpl_hook_aggregate'> & {
  hooks: ContextHook[];
};

type IgpHook = typed<'hpl_igp'> & { oracle: typed<'hpl_igp_oracle'> };

function isIgpHookType(hook: ContextHook): hook is IgpHook {
  return hook.type === 'hpl_igp';
}

function getIsmsMultisigConfig() {
  if (!config.deploy.ism || config.deploy.ism.type != "multisig") {
    throw new Error('Ism multisig config not found');
  }
  return config.deploy.ism;
}

function findIgpHookInAggregate() {
  const defaultHook = config.deploy.hooks?.default;
  if (defaultHook && defaultHook.type === 'aggregate') {
    const igpHook = defaultHook.hooks.find(
      (hook): hook is IgpHookType => hook.type === 'igp'
    );
    if (!igpHook) {
      throw new Error('igpHook not found under aggregate hook');
    }
    return igpHook;
  }
  throw new Error('Aggregate igp hook not found');
}

async function handleRegisterIsm(_: object, cmd: Command) {
  const { ctx, client } = CONTAINER.get(Dependencies);

  if (!ctx.deployments.isms || ctx.deployments.isms.type != "hpl_ism_multisig") {
    throw new Error('Ism multisig context not found');
  }
  const ismMultisigAddress = ctx.deployments.isms.address
  const ismsConfig = getIsmsMultisigConfig();

  const multisig = {
    type: 'hpl_ism_multisig',
    address: ismMultisigAddress as string,
  };  
  await executeMultiMsg(
    client,
    Object.entries(ismsConfig.validators).map(([domain, { addrs, threshold }]) => ({
      contract: multisig,
      msg: {
        set_validators: {
          domain: Number(domain),
          threshold,
          validators: addrs.map((v) =>
            v === '<signer>' ? client.signer_addr : v
          ),
        },
      },
    }))
  );
  }

async function handleRegisterIgpOracle(_: object, cmd: Command) {
  const { ctx, client } = CONTAINER.get(Dependencies);

  const defaultHook = ctx.deployments.hooks?.default;
  let igpHookDeployment;
  let aggregateHook;
  let igpOracleAddress: string | undefined;
  if (defaultHook && defaultHook.type === 'hpl_hook_aggregate') {
    igpHookDeployment = defaultHook.hooks.find(isIgpHookType) as IgpHook;
    igpOracleAddress = igpHookDeployment?.oracle?.address;
    aggregateHook = defaultHook as AggregateHook;
  }
  if (!igpHookDeployment) {
    throw new Error('igpHook is undefined in context');
  }
  if (!igpOracleAddress) {
    throw new Error('igpOracleAddress is undefined in context');
  }
  if (!aggregateHook) {
    throw new Error('aggregateHook is undefined in context');
  }

  const igpType: IgpHook | undefined = aggregateHook?.hooks.find(
    (hook): hook is IgpHook => hook.type === 'hpl_igp'
  );
  if (!igpType) {
    throw new Error('igpType is undefined');
  }

  const igpHookConfig = findIgpHookInAggregate();
  await executeMultiMsg(client, [
    {
      contract: {
        type: 'hpl_igp_oracle',
        address: igpOracleAddress,
      },
      msg: {
        set_remote_gas_data_configs: {
          configs: Object.entries(igpHookConfig.configs).map(([domain, v]) => ({
            remote_domain: Number(domain),
            token_exchange_rate: v.exchange_rate.toString(),
            gas_price: v.gas_price.toString(),
          })),
        },
      },
    },
    {
      contract: {
        type: 'hpl_igp',
        address: igpHookDeployment.address,
      },
      msg: {
        router: {
          set_routes: {
            set: Object.keys(igpHookConfig.configs).map((domain) => ({
              domain: Number(domain),
              route: igpOracleAddress,
            })),
          },
        },
      },
    },
  ]);
}
