import { injectable } from "inversify";
import { Context, HplMailboxInstantiateMsg, Migration } from "../types";
import HplMailbox from "../contracts/hpl_mailbox";
import HplIsmMultisig from "../contracts/hpl_ism_multisig";


@injectable()
export default class MailboxMigration implements Migration {
  name: string = "mailbox";
  after: string = "initialize_standalone";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private ism_multisig: HplIsmMultisig,
  ) {}

  run = async (): Promise<Context> => {
    const mailboxInit: HplMailboxInstantiateMsg = {
      owner: this.ctx.address!,
      default_ism: this.ism_multisig.address!,
    }
    this.ctx.contracts[this.mailbox.contractName] = await this.mailbox.instantiate(mailboxInit);
    return this.ctx;
  }

  setContext = (ctx: Context) => { this.ctx = ctx }
}
