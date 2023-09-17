use common::token::Token;
use cosmwasm_schema::{cw_serde, QueryResponses};

/// This struct contains required variables to instantiate a new contract.
#[cw_serde]
pub struct InstantiateMsg { }

/// This enum describes available contract's execution messages.
#[cw_serde]
pub enum ExecuteMsg {
    /// Allows to update the contract's `allowed_token`. Only owner can update.
    BurnToken { },
}

/// This enum describes available contract's query messages.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg { }
