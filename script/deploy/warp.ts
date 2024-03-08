import { deployContract } from '../shared/contract';
import { Dependencies } from '../shared/ioc';
import { orSigner } from '../shared/utils';

type NativeTokenBridged = {
  denom: string;
  metadata?: {
    description: string;
    denom_units: {
      denom: string;
      exponent: string;
      aliases: string[];
    }[];
    base: string;
    display: string;
    name: string;
    symbol: string;
  };
};

type NativeTokenCollateral = {
  denom: string;
};

type Cw20TokenBridged = {
  code_id: number;
  init_msg: {
    name: string;
    symbol: string;
    decimals: number;
    initial_balances: { address: string; amount: string }[];
    mint?: { minter: string; cap?: string };
    marketing?: {
      project?: string;
      description?: string;
      marketing?: string;
      logo?: { url: string } & {
        embedded: { svg: string } & { png: string };
      };
    };
  };
};

type Cw20TokenCollateral = {
  address: string;
};

type WarpTokenConfigMap = {
  native: {
    bridged: { bridged: NativeTokenBridged };
    collateral: { collateral: NativeTokenCollateral };
  };
  cw20: {
    bridged: { bridged: Cw20TokenBridged };
    collateral: { collateral: Cw20TokenCollateral };
  };
};

export type WarpTokenConfig<
  TokenType extends 'native' | 'cw20' = 'native' | 'cw20',
  TokenMode extends 'bridged' | 'collateral' = 'bridged' | 'collateral',
  OwnerType extends string | '<signer>' = '<signer>',
> = {
  type: TokenType;
  mode: TokenMode;

  id: string;
  owner: OwnerType;
  config: WarpTokenConfigMap[TokenType][TokenMode];
};

export async function deployNativeTokenWarp(
  { ctx, client, network }: Dependencies,
  mailbox: string,
  config: WarpTokenConfig<'native'>,
): Promise<{ type: 'hpl_warp_native'; address: string } | undefined> {
  const deployments = ctx.deployments;

  const preload = deployments?.warp?.native?.find((v) => v.id === config.id);
  if (preload) {
    console.error(
      '[error]'.red,
      `warp route ${preload.id} already exists.`,
      `type: ${preload.type},`,
      `addr: ${preload.address}`,
    );
    return;
  }

  const nativeWarp = await deployContract(ctx, client, 'hpl_warp_native', {
    token: config.config,

    hrp: network.hrp,
    owner: orSigner(client, config.owner),
    mailbox,
  });

  return nativeWarp;
}

export async function deployCw20TokenWarp(
  { ctx, client, network }: Dependencies,
  mailbox: string,
  config: WarpTokenConfig<'cw20'>,
): Promise<{ type: 'hpl_warp_cw20'; address: string } | undefined> {
  const deployments = ctx.deployments;

  const preload = deployments?.warp?.cw20?.find((v) => v.id === config.id);
  if (preload) {
    console.error(
      '[error]'.red,
      `warp route ${preload.id} already exists.`,
      `type: ${preload.type},`,
      `addr: ${preload.address}`,
    );
    return;
  }

  const cw20Warp = await deployContract(ctx, client, 'hpl_warp_cw20', {
    token: config.config,

    hrp: network.hrp,
    owner: orSigner(client, config.owner),
    mailbox,
  });

  return cw20Warp;
}
