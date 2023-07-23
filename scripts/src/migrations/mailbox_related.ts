import { injectable } from "inversify";
import { Context, HplIsmRoutingInstantiateMsg, HplMulticallInstantiateMsg, HplValidatorAnnounceInstantiateMsg, Migration } from "../types";
import HplMailbox from "../contracts/hpl_mailbox";
import HplIsmRouting from "../contracts/hpl_ism_routing";
import HplMulticall from "../contracts/hpl_multicall";
import HplValidatorAnnounce from "../contracts/hpl_validator_announce";


@injectable()
export default class MailboxMigration implements Migration {
  name: string = "mailbox_related";
  after: string = "mailbox";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private ismRouting: HplIsmRouting,
    private multicall: HplMulticall,
    private va: HplValidatorAnnounce,
  ) {}

  run = async (): Promise<Context> => {

    const routingMsgs: HplIsmRoutingInstantiateMsg = {
      owner: this.ctx.address!,
      isms: [{
        domain: 4337,
        address: this.mailbox.address!,
      }]
    }
    this.ctx.contracts[this.ismRouting.contractName] = await this.ismRouting.instantiate(routingMsgs);

    const multicallMsg: HplMulticallInstantiateMsg = {
      owner: this.ctx.address!,
      mailbox: this.mailbox.address!,
    }
    this.ctx.contracts[this.multicall.contractName] = await this.multicall.instantiate(multicallMsg);

    const vaMsg: HplValidatorAnnounceInstantiateMsg = {
      addr_prefix: "osmo",
      mailbox: this.mailbox.address!,
      local_domain: 4337,
    }
    this.ctx.contracts[this.va.contractName] = await this.va.instantiate(vaMsg);

    return this.ctx;
  }

  setContext = (ctx: Context) => { this.ctx = ctx }
}
