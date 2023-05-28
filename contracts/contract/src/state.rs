/*
The state.rs file typically contains the definition of the contract's state struct and associated
functions for manipulating and accessing the contract state.
 */
use common::token::Token;
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

/// This struct contains configuration parameters for the contract.
#[cw_serde]
pub struct Config {
    /// Token allowed inside the contarct.
    pub token: Token,
}

pub const CONFIG: Item<Config> = Item::new("config");
