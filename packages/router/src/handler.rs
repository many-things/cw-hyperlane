use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Order, QueryResponse, Response,
    StdError, StdResult, Storage,
};
use hpl_interface::router::{DomainsResponse, RouterMsg, RouterQuery, RouterResponse, RouterSet};

use crate::state::ROUTES;

pub fn handle(deps: DepsMut, _env: Env, info: MessageInfo, msg: RouterMsg) -> StdResult<Response> {
    use RouterMsg::*;

    match msg {
        EnrollRemoteRouter { set } => {
            enroll_remote_router(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::enroll_remote_router")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "set",
                        serde_json_wasm::to_string(&set)
                            .map_err(|_| StdError::generic_err("encoding failed"))?,
                    ),
            );

            Ok(resp)
        }
        EnrollRemoteRouters { set } => {
            enroll_remote_routers(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::enroll_remote_routers")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "set",
                        serde_json_wasm::to_string(&set)
                            .map_err(|_| StdError::generic_err("encoding failed"))?,
                    ),
            );

            Ok(resp)
        }
    }
}

pub fn enroll_remote_router(storage: &mut dyn Storage, set: RouterSet) -> StdResult<()> {
    ROUTES.save(storage, set.domain, &set.router)?;

    Ok(())
}

pub fn enroll_remote_routers(storage: &mut dyn Storage, set: Vec<RouterSet>) -> StdResult<()> {
    for RouterSet { domain, router } in set {
        ROUTES.save(storage, domain, &router)?;
    }

    Ok(())
}

pub fn handle_query(deps: Deps, _env: Env, msg: RouterQuery) -> StdResult<QueryResponse> {
    match msg {
        RouterQuery::Domains {} => to_binary(&DomainsResponse {
            domains: get_domains(deps.storage)?,
        }),
        RouterQuery::Router { domain } => to_binary(&RouterResponse {
            router: get_router(deps.storage, domain)?,
        }),
    }
}

pub fn is_router(storage: &dyn Storage, domain: u32, router: Binary) -> StdResult<bool> {
    Ok(router == ROUTES.load(storage, domain)?)
}

pub fn get_domains(storage: &dyn Storage) -> StdResult<Vec<u32>> {
    ROUTES.keys(storage, None, None, Order::Ascending).collect()
}

pub fn get_router(storage: &dyn Storage, domain: u32) -> StdResult<Binary> {
    Ok(ROUTES.load(storage, domain).unwrap_or_default())
}
