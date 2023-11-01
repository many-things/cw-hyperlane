import { injectable } from "inversify";
import { Context, Migration } from "../types";
import { HplIsmRouting, HplMailbox, HplValidatorAnnounce } from "../contracts";

@injectable()
export default class MailboxMigration implements Migration {
  name: string = "mailbox_related";
  after: string = "mailbox";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private ismRouting: HplIsmRouting,
    private va: HplValidatorAnnounce
  ) {}

  run = async (): Promise<Context> => {
    // const routingMsgs: HplIsmRoutingInstantiateMsg = {
    //   owner: this.ctx.address!,
    //   isms: [
    //     {
    //       domain: 4337,
    //       address: this.mailbox.address!,
    //     },
    //   ],
    // };
    // this.ctx.contracts[this.ismRouting.contractName] =
    //   await this.ismRouting.instantiate(routingMsgs);

    // const vaMsg: HplValidatorAnnounceInstantiateMsg = {
    //   addr_prefix: "osmo",
    //   mailbox: this.mailbox.address!,
    //   local_domain: 4337,
    // };
    // this.ctx.contracts[this.va.contractName] = await this.va.instantiate(vaMsg);

    return this.ctx;
  };

  setContext = (ctx: Context) => {
    this.ctx = ctx;
  };
}
