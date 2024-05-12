use thiserror::Error;
use cosmrs::ErrorReport;
use prost::{DecodeError, EncodeError};
use cosmrs::tendermint::Error as TendermintError;
use tonic::Status;

#[derive(Error,Debug)]
pub enum QueryError{
    #[error("Unknown Account Type")]
    UnknownAccountType,
    #[error("Simulation Failed")]
    SimulationFailed,
    #[error("Tx Timeout")]
    TxTimeout,
    #[error("Tx error: {0}")]
    TxError(String),
    #[error("Decode Error")]
    ProstDecodeError(#[from] DecodeError),
    #[error("Encode Error")]
    ProstEncodeError(#[from] EncodeError),
    #[error("No base account")]
    NoVestingBaseAccount,
    #[error("ErrorReport")]
    ErrorReport(#[from] ErrorReport),
    #[error("Tendermint Error")]
    TendermintError(#[from] TendermintError),
    #[error("Tonic Error")]
    StatusError(#[from] Status),
    #[error("Txn not found")]
    TxnNotFound,
    #[error("Fetch Error")]
    ReqwestError(#[from] reqwest::Error),
}