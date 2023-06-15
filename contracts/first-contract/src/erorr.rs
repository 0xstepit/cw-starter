/*
The **error.rs** file typically contains the definition of custom error types and associated
functions for handling and propagating errors within the contract.
*/
use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    Ownership(#[from] OwnershipError),
}
