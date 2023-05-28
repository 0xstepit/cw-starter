use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::token::Token;

/// Represents a coin, which can be either a native token or a CW20 token with an amount.
#[cw_serde]
pub struct Coin {
    pub denom: Token,
    pub amount: Uint128,
}

impl Coin {}
