/*
The **contract.rs** file typically contains the main contract code and logic. It serves as the entry
 point for executing the contract's operations and implementing its functionality.
*/

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    erorr::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG},
};

pub const CONTRACT_NAME: &str = "crates.io/cw-template";
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
        Some(deps.api.addr_validate(&msg.owner)?.as_ref()),
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
        UpdateOwnership(action) => execute::update_owner(deps, env, &info.sender, action),
        UpdateToken { new_token } => execute::update_token(deps, &info.sender, new_token),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        Ownership {} => to_binary(&cw_ownable::get_ownership(deps.storage)?),
        AllowedToken {} => to_binary(&query::allowed_token(deps)?),
    }
}

pub mod execute {
    use common::token::Token;
    use cosmwasm_std::{Addr, StdError};
    use cw_ownable::Action;

    use super::*;

    pub fn update_owner(
        deps: DepsMut,
        env: Env,
        sender: &Addr,
        action: Action,
    ) -> Result<Response, ContractError> {
        cw_ownable::update_ownership(deps, &env.block, sender, action)?;
        Ok(Response::default())
    }

    pub fn update_token(
        deps: DepsMut,
        sender: &Addr,
        new_token: Token,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;

        CONFIG.update(deps.storage, |mut c| -> Result<_, StdError> {
            c.token = new_token;
            Ok(c)
        })?;
        Ok(Response::new())
    }
}

pub mod query {
    use common::token::Token;

    use super::*;

    pub fn allowed_token(deps: Deps) -> StdResult<Token> {
        Ok(CONFIG.load(deps.storage)?.token)
    }
}

// -------------------------------------------------------------------------------------------------
// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use common::token::Token;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };

    use crate::msg::{ExecuteMsg, InstantiateMsg};

    use super::*;

    const NATIVE: &str = "atom";
    const CW20: &str = "cosmic000";
    const OWNER: &str = "0xstepit000";

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("stepit", &[]);

        instantiate(
            deps.as_mut(),
            env,
            info,
            InstantiateMsg {
                owner: OWNER.to_string(),
                allowed_token: Token::Native(NATIVE.to_string()),
            },
        )
        .unwrap();

        let owner = cw_ownable::get_ownership(deps.as_ref().storage)
            .unwrap()
            .owner;
        assert_eq!(
            owner,
            Some(Addr::unchecked(OWNER)),
            "expected owner sent in message"
        )
    }

    #[test]
    fn update_token() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("stepit", &[]);

        instantiate(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            InstantiateMsg {
                owner: OWNER.to_string(),
                allowed_token: Token::Native(NATIVE.to_string()),
            },
        )
        .unwrap();

        let exec_msg = ExecuteMsg::UpdateToken {
            new_token: Token::Cw20(CW20.to_string()),
        };
        let q_msg = QueryMsg::AllowedToken {};

        {
            execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap_err();
        }

        {
            let info = mock_info(OWNER, &[]);
            execute(deps.as_mut(), env.clone(), info, exec_msg).unwrap();

            let resp: Token = from_binary(&query(deps.as_ref(), env, q_msg).unwrap()).unwrap();
            assert_eq!(
                resp,
                Token::Cw20(CW20.to_string()),
                "expected token updated since sender is owner"
            )
        }
    }
}
