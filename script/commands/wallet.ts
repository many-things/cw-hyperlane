import {
  DirectSecp256k1HdWallet,
  DirectSecp256k1Wallet,
  makeCosmoshubPath,
} from '@cosmjs/proto-signing';
import { Command } from 'commander';

import { getKeyPair } from '../shared/crypto';
import { CONTAINER, Dependencies } from '../shared/ioc';
import { addPad, extractByte32AddrFromBech32 } from '../shared/utils';

const walletCmd = new Command('wallet')
  .description('Wallet commands')
  .configureHelp({ showGlobalOptions: true });

walletCmd
  .command('new')
  .description('Create a new wallet')
  .action(async () => {
    const deps = CONTAINER.get(Dependencies);

    const prefix = { prefix: deps.network.hrp };
    const wallet = await DirectSecp256k1HdWallet.generate(12, prefix);

    const [account] = await wallet.getAccounts();
    const { privkey } = await getKeyPair(wallet.mnemonic, makeCosmoshubPath(0));

    console.log({
      address: account.address,
      mnemonic: wallet.mnemonic,
      privateKey: Buffer.from(privkey).toString('hex'),
    });
  });

walletCmd
  .command('address')
  .description('Get the address of the wallet')
  .option('--private-key <private-key>', 'The private key of the wallet')
  .option('--mnemonic <mnemonic>', 'The mnemonic of the wallet')
  .action(async (_, cmd) => {
    const opts = cmd.optsWithGlobals();
    const deps = CONTAINER.get(Dependencies);

    if (
      (opts.privateKey && opts.mnemonic) ||
      (!opts.privateKey && !opts.mnemonic)
    ) {
      throw new Error(
        'Only one of --private-key and --mnemonic can be specified',
      );
    }

    const wallet = opts.privateKey
      ? await DirectSecp256k1Wallet.fromKey(
          Buffer.from(
            opts.privateKey.startsWith('0x')
              ? opts.privateKey.slice(2)
              : opts.privateKey,
            'hex',
          ),
          deps.network.hrp,
        )
      : await DirectSecp256k1HdWallet.fromMnemonic(opts.mnemonic, {
          prefix: deps.network.hrp,
        });

    const [account] = await wallet.getAccounts();

    console.log(account.address);
  });

walletCmd
  .command('zero-pad')
  .argument('address', 'eth address of length 20 bytes')
  .description('zero pads an ETH address to length 64 bytes')
  .action((address: string) => console.log(`0x${addPad(address)}`));

walletCmd
  .command('convert-cosmos-to-eth')
  .argument(
    'address',
    'converts a bech32 cosmos address to a 64 byte length eth address',
  )
  .description('zero pads an ETH address to length 64 bytes')
  .action((address: string) =>
    console.log(`0x${extractByte32AddrFromBech32(address)}`),
  );

export { walletCmd };
