/*
The **msg**.rs file typically contains the definition of the contract's message struct used in execute
and query requests.
*/

use common::token::Token;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

/// This struct contains required variables to instantiate a new contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// Contract owner address
    pub owner: String,
    /// Representes the tokens users are allowed to deposit.
    pub allowed_token: Token,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    UpdateToken { new_token: Token },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns[Token]]
    AllowedToken {},
}
