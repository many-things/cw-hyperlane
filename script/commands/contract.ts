import { Command } from 'commander';

import { getNetwork } from '../shared/config';
import { contractNames } from '../shared/constants';
import { executeContract } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';
import { addPad } from '../shared/utils';

export const contractCmd = new Command('contract').configureHelp({
  showGlobalOptions: true,
});

contractCmd.command('list').action(() => {
  console.log('Available contracts:'.green);
  contractNames.forEach((v) => console.log('-', v));
});

contractCmd
  .command('test-dispatch')
  .argument('dest-domian')
  .argument('recipient-addr')
  .argument('msg-body')
  .action(async (destDomain, recipientAddr, msgBody, _, cmd) => {
    const opts = cmd.optsWithGlobals();
    const { ctx, client } = CONTAINER.get(Dependencies);
    const network = getNetwork(opts.networkId);

    const mailbox = ctx.deployments.core?.mailbox!;

    const res = await executeContract(
      client,
      mailbox,
      {
        dispatch: {
          dest_domain: parseInt(destDomain),
          recipient_addr: addPad(recipientAddr),
          msg_body: Buffer.from(msgBody, 'utf-8').toString('hex'),
        },
      },
      [{ amount: '500', denom: network.gas.denom }],
    );

    console.log(res.hash);
  });
