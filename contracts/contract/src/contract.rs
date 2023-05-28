/*
The **contract.rs** file typically contains the main contract code and logic. It serves as the entry
 point for executing the contract's operations and implementing its functionality.
*/

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_ownable::Ownership;

use crate::{
    erorr::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG},
};

pub const CONTRACT_NAME: &str = "crates.io/contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(
        deps.storage,
        deps.api,
        Some(&deps.api.addr_validate(&msg.owner)?.to_string()),
    )?;

    CONFIG.save(
        deps.storage,
        &Config {
            token: msg.allowed_token,
        },
    )?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        UpdateOwnership(action) => unimplemented!(),
        UpdateToken { new_token } => execute::update_token(new_token),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        Ownership {} => to_binary(&cw_ownable::get_ownership(deps.storage)?),
        AllowedToken {} => to_binary(&query::allowed_token()?),
    }
}

pub mod execute {
    use common::token::Token;

    use super::*;

    pub fn update_token(new_token: Token) -> Result<Response, ContractError> {
        unimplemented!()
    }
}

pub mod query {
    use common::token::Token;

    use super::*;

    pub fn allowed_token() -> StdResult<Token> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(tests)]
mod tests {

    #[test]
    fn proper_instantiation() {}
}
