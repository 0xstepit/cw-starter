use cosmwasm_schema::cw_serde;

/// Represents a token, which can be either a native token or a CW20 token.
#[cw_serde]
pub enum Token {
    /// Variant to indicate a native token with its denom.
    Native(String),
    /// Variant to indicate a Cw20 token with its cw20 contract address
    Cw20(String),
}

impl Token {}
