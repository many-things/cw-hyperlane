import { encodePubkey } from '@cosmjs/proto-signing';

import { MULTISIG_MEMBERS } from './deps';
import { executeContract } from './shared';
import { ClientSet, Member } from './types';

export type MultisigAddress = {
  hub: string;
  prop: string;
  preProp: string;
};

type CreateMultisigOptions = {
  address: MultisigAddress;
  members: string[];
  client: ClientSet;
};

export class Multisig {
  private constructor(
    public readonly address: MultisigAddress,
    public readonly members: Member[],
  ) {}

  async run(msgs: object) {
    const [runner] = this.members;

    const { proposalId, resp: proposeResp } = await this.propose(runner, msgs);
    console.log(`Proposal ID: ${proposalId}. txHash: ${proposeResp.hash}`);

    await Promise.all(this.members.map((v) => this.vote(v, proposalId, 'yes')));
    console.log('Executing proposal...');

    const execResp = await this.exec(runner, proposalId);
    console.log(`Proposal executed! txHash: ${execResp.hash}`);
  }

  async exec(member: Member, proposalId: number) {
    return executeContract(member, this.address.prop, {
      execute: {
        proposal_id: proposalId,
      },
    });
  }

  async vote(member: Member, proposalId: number, vote: 'yes' | 'no') {
    return executeContract(member, this.address.prop, {
      vote: {
        proposal_id: proposalId,
        vote,
      },
    });
  }

  async propose(member: Member, msgs: object) {
    const proposalId = await member.client.wasm.queryContractSmart(
      this.address.prop,
      {
        proposal_count: {},
      },
    );
    const nextProposalId = Number(proposalId) + 1;

    const resp = await executeContract(member, this.address.preProp, {
      propose: {
        msg: {
          propose: {
            title: 'title',
            description: 'description',
            msgs,
          },
        },
      },
    });

    return { proposalId: nextProposalId, resp };
  }

  static async create(opts: CreateMultisigOptions): Promise<Multisig> {
    const members = await Promise.all(
      MULTISIG_MEMBERS.map(async (v) => {
        const account = await opts.client.stargate.getAccount(v);
        if (!account) throw new Error(`Account ${v} not found`);
        if (!account.pubkey) throw new Error(`Account ${v} has no pubkey`);

        return {
          address: v,
          pubkey: encodePubkey({
            type: 'tendermint/PubKeySecp256k1',
            value: account.pubkey.value,
          }),
          client: opts.client,
        };
      }),
    );

    return new Multisig(opts.address, members);
  }
}
