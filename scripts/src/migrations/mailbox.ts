import { injectable } from "inversify";
import { Context, HplMailboxInstantiateMsg, Migration } from "../types";
import { HplIsmMultisig, HplMailbox } from "../contracts";

@injectable()
export default class MailboxMigration implements Migration {
  name: string = "mailbox";
  after: string = "initialize_standalone";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private ism_multisig: HplIsmMultisig
  ) {}

  run = async (): Promise<Context> => {
    const mailboxInit: HplMailboxInstantiateMsg = {
      owner: this.ctx.address!,
      hrp: "dual",
      domain: 33333,
    };
    this.ctx.contracts[this.mailbox.contractName] =
      await this.mailbox.instantiate(mailboxInit);
    return this.ctx;
  };

  setContext = (ctx: Context) => {
    this.ctx = ctx;
  };
}
