use cosmwasm_std::{Addr, Deps};
use hpl_interface::core::mailbox;

use crate::error::ContractError;

pub fn local_domain(deps: Deps, mailbox: &Addr) -> Result<u32, ContractError> {
    let res: mailbox::LocalDomainResponse = deps
        .querier
        .query_wasm_smart(mailbox, &mailbox::MailboxQueryMsg::LocalDomain {})?;

    Ok(res.local_domain)
}
