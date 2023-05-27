use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;
use package::token::Token;


#[cw_serde]
pub struct Config {
    asset: Token
}

pub const CONFIG: Item<Config> = Item::new("config");