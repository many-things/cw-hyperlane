import fs from 'fs';
import path from 'path';

import { defaultContextPath } from './constants';
import { ContractNames } from './contract';

export type typed<T extends ContractNames> = {
  type: T;
  address: string;
  hexed: string;
};

export type ContextIsm =
  | (typed<'hpl_ism_aggregate'> & {
      isms: ContextIsm[];
    })
  | typed<'hpl_ism_multisig'>
  | typed<'hpl_ism_pausable'>
  | (typed<'hpl_ism_routing'> & {
      isms: Record<number, ContextIsm>;
    })
  | typed<'hpl_test_mock_ism'>;

export type ContextHook =
  | (typed<'hpl_igp'> & { oracle: typed<'hpl_igp_oracle'> })
  | (typed<'hpl_hook_aggregate'> & {
      hooks: ContextHook[];
    })
  | typed<'hpl_hook_fee'>
  | typed<'hpl_hook_merkle'>
  | typed<'hpl_hook_pausable'>
  | (typed<'hpl_hook_routing'> & {
      hooks: Record<number, ContextHook>;
    })
  | (typed<'hpl_hook_routing_custom'> & {
      hooks: Record<number, Record<string | 'default', ContextHook>>;
    })
  | (typed<'hpl_hook_routing_fallback'> & {
      hooks: Record<number | 'fallback', ContextHook>;
    })
  | typed<'hpl_test_mock_hook'>;

export type ContextDeployments = {
  core?: {
    mailbox?: typed<'hpl_mailbox'>;
    validator_announce?: typed<'hpl_validator_announce'>;
  };

  isms?: ContextIsm;

  hooks?: {
    default?: ContextHook;
    required?: ContextHook;
  };

  warp?: {
    cw20?: ({ id: string } & typed<'hpl_warp_cw20'>)[];
    native?: ({ id: string } & typed<'hpl_warp_native'>)[];
  };

  test?: {
    msg_receiver?: typed<'hpl_test_mock_msg_receiver'>;
  };
};

export class Context {
  artifacts: Record<ContractNames, number>;
  deployments: ContextDeployments;

  latestMigration?: string;
}

export function loadContext(
  network: string,
  { contextPath }: { contextPath: string } = {
    contextPath: defaultContextPath,
  },
): Context {
  try {
    const fileName = path.join(contextPath, `${network}.json`);
    const result = fs.readFileSync(fileName, 'utf-8');
    return JSON.parse(result.toString()) as Context;
  } catch (err) {
    console.error('Failed to load context. Returning an empty context object.');
  }

  return {
    artifacts: {},
    deployments: {},
  };
}

export function saveContext(
  network: string,
  context: Context,
  { contextPath }: { contextPath: string } = {
    contextPath: defaultContextPath,
  },
) {
  fs.mkdirSync(contextPath, { recursive: true });
  const fileName = path.join(contextPath, `${network}.json`);
  fs.writeFileSync(fileName, JSON.stringify(context, null, 2));
}
