import { Command } from 'commander';

import { IgpHookType, config } from '../shared/config';
import { typed, ContextHook } from '../shared/context';
import { executeMultiMsg } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';

export const registerCmd = new Command('register')
  .description('Register new chain to contracts')
  .configureHelp({ showGlobalOptions: true });

registerCmd.command('igporacle')
  .action(handleRegisterIgpOracle);
registerCmd.command('ism-multisig')
  .action(handleRegisterIsm);

type AggregateHook = (typed<'hpl_hook_aggregate'> & {
hooks: ContextHook[];
})

type IgpHook = (typed<'hpl_igp'> & { oracle: typed<'hpl_igp_oracle'> });

function isIgpHookType(hook: ContextHook): hook is IgpHook {
    return hook.type === 'hpl_igp';
};

function findIsmsConfig() {
    return config.deploy.ism
};

function findIgpHookInAggregate() {
    const defaultHook = config.deploy.hooks?.default;
    if (defaultHook && defaultHook.type === 'aggregate') {
        const igpHook = defaultHook.hooks.find((hook): hook is IgpHookType => hook.type === 'igp');
        return igpHook;
    }
    return undefined;
};

async function handleRegisterIsm(_: object, cmd: Command) {
    const { ctx, client } = CONTAINER.get(Dependencies);

    const ismMultisig = ctx.deployments.isms;
    let ismMultisigAddress: string | undefined;
    if (ismMultisig && ismMultisig.type === 'hpl_ism_multisig') {
        ismMultisigAddress = ismMultisig.address
    }
    if (!ismMultisigAddress) {
        throw new Error("Ism multisig context not found");
    }
    const ismsConfig = findIsmsConfig();
    if (!ismsConfig) {
        throw new Error ("Ism multisig config not found")
    }
    switch (ismsConfig.type) {
        // deploy multisig ism
        case 'multisig': {
            const multisig = {
                type: 'hpl_ism_multisig',
                address: ismMultisigAddress as string
            };
            await executeMultiMsg(
                client,
                Object.entries(ismsConfig.validators).map(
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

        default: {
            throw new Error('Multisig ism config not found')
        }
    };
};

async function handleRegisterIgpOracle(_: object, cmd: Command) {
    const { ctx } = CONTAINER.get(Dependencies);


    const defaultHook = ctx.deployments.hooks?.default;
    let igpHook;
    let aggregateHook;
    let igpOracleAddress: string | undefined;
    if (defaultHook && defaultHook.type === 'hpl_hook_aggregate') {
        igpHook = defaultHook.hooks.find(isIgpHookType) as IgpHook;
        igpOracleAddress = igpHook ? igpHook.oracle?.address : undefined;
        aggregateHook = defaultHook as AggregateHook;
    }
    if (!igpHook) {
        throw new Error('igpHook is undefined in context');
    }
    if (!igpOracleAddress) {
        throw new Error('igpOracleAddress is undefined in context');
    }
    if (!aggregateHook) {
        throw new Error('aggregateHook is undefined in context');
    }

    const igpType: IgpHook | undefined = aggregateHook?.hooks.find(
        (hook): hook is IgpHook => hook.type === 'hpl_igp');

    // Ensure igpType is defined
    if (!igpType) {
        throw new Error('igpType is undefined');
    }

    const igpHookConfig = findIgpHookInAggregate();
    if (!igpHookConfig) {
        throw new Error('No IGP hook found in aggregate');
    }
    await executeMultiMsg(client, [
        {
            contract: {
                type: "hpl_igp",
                address: igpOracleAddress
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
                type: "hpl_igp",
                address: igpHook.address,
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
};