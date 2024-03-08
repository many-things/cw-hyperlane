import { CodeDetails } from '@cosmjs/cosmwasm-stargate';
import { Command } from 'commander';

import { ContextHook, ContextIsm } from '../shared/context';
import { ContractNames } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';
import { askQuestion, waitTx } from '../shared/utils';

export const migrateCmd = new Command('migrate')
  .description('Migrate contracts')
  .configureHelp({ showGlobalOptions: true })
  .option('-c --contracts <contracts...>', 'specify contracts to migrate')
  .action(handleMigrate);

async function handleMigrate(_: any, cmd: any) {
  const { contracts } = cmd.optsWithGlobals();

  const { ctx, client } = CONTAINER.get(Dependencies);

  const flatten = [
    ...flattenIsm(ctx.deployments.isms),
    ...flattenHook(ctx.deployments.hooks?.default),
    ...flattenHook(ctx.deployments.hooks?.required),
    ctx.deployments.core?.mailbox,
    ctx.deployments.core?.validator_announce,
    ctx.deployments.test?.msg_receiver,
    ...(ctx.deployments.warp?.cw20 || []),
    ...(ctx.deployments.warp?.native || []),
  ].filter((v, i, arr) => arr.indexOf(v) === i) as {
    type: ContractNames;
    address: string;
  }[];

  const withContractInfo = await Promise.all(
    (contracts
      ? flatten.filter((v) => contracts.includes(v.type))
      : flatten
    ).map(async (v) => {
      const contractInfo = await client.wasm.getContract(v.address);
      const codeInfo = await client.wasm.getCodeDetails(contractInfo.codeId);
      return { ...v, contractInfo, codeInfo };
    }),
  );

  const artifacts = Object.fromEntries(
    await Promise.all(
      Object.entries(ctx.artifacts).map(async ([contractName, codeId]) => {
        const codeInfo = await client.wasm.getCodeDetails(codeId);
        return [contractName, codeInfo];
      }),
    ),
  ) as Record<ContractNames, CodeDetails>;

  const toMigrate = withContractInfo.filter(
    (v) =>
      v.codeInfo.id !== ctx.artifacts[v.type] &&
      v.codeInfo.checksum !== artifacts[v.type].checksum,
  );

  if (toMigrate.length === 0) {
    console.log('No changes detected.');
    return;
  }

  if (!(await askQuestion('Do you want to proceed? (y/n)'))) {
    console.log('Aborted.');
    return;
  }
  console.log('Proceeding to migrate...');

  for (const migrate of toMigrate) {
    const res = await client.wasm.migrate(
      client.signer,
      migrate.address,
      artifacts[migrate.type].id,
      {},
      'auto',
    );
    await waitTx(res.transactionHash, client.stargate);
    console.log(`${migrate.type} migrated successfully`);
  }
}

const flattenIsm = (
  ism: ContextIsm | undefined,
): { type: ContractNames; address: string }[] => {
  if (!ism) return [];

  switch (ism.type) {
    case 'hpl_ism_aggregate':
      return [
        { type: ism.type, address: ism.address },
        ...ism.isms.flatMap(flattenIsm),
      ];
    case 'hpl_ism_routing':
      return [
        { type: ism.type, address: ism.address },
        ...Object.values(ism.isms).flatMap(flattenIsm),
      ];
    case 'hpl_ism_multisig':
      return [ism];
    case 'hpl_ism_pausable':
      return [ism];
    case 'hpl_test_mock_ism':
      return [ism];
  }
};

const flattenHook = (
  hook: ContextHook | undefined,
): { type: ContractNames; address: string }[] => {
  if (!hook) return [];

  switch (hook.type) {
    case 'hpl_hook_aggregate':
      return [
        { type: hook.type, address: hook.address },
        ...hook.hooks.flatMap(flattenHook),
      ];
    case 'hpl_hook_routing':
      return [
        { type: hook.type, address: hook.address },
        ...Object.values(hook.hooks).flatMap(flattenHook),
      ];
    case 'hpl_hook_routing_custom':
      return [
        { type: hook.type, address: hook.address },
        ...Object.values(hook.hooks).flatMap((v) =>
          Object.values(v).flatMap(flattenHook),
        ),
      ];
    case 'hpl_hook_routing_fallback':
      return [
        { type: hook.type, address: hook.address },
        ...Object.values(hook.hooks).flatMap(flattenHook),
      ];

    case 'hpl_igp':
      return [{ type: hook.type, address: hook.address }, hook.oracle];
    case 'hpl_hook_fee':
      return [hook];
    case 'hpl_hook_merkle':
      return [hook];
    case 'hpl_hook_pausable':
      return [hook];
    case 'hpl_test_mock_hook':
      return [hook];
  }
};
