use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("No data stored")]
    NoDataStored {},

    #[error("Invalid data")]
    InvalidData {},

    #[error("Invalid funds")]
    InvalidFunds {
        expected: Uint128,
        got: Uint128,
    },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
