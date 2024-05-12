pub use query_client::error::QueryError;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum WalletError{
    #[error("Cannot open file")]
    FileOpenError,
    #[error("Io Error")]
    IoError(#[from] std::io::Error),
    #[error("Error in getting balances")]
    BalanceError,
    #[error("Decode Hex Error")]
    HexError(#[from] hex::FromHexError),
    #[error("Account Error {0}")]
    AccountError(String),
    #[error("Password not set")]
    PasswordError(String),
    #[error("File Credential Error")]
    FileCredentialError(#[from] rust_keyring::error::FileCredentialError),
    #[error("User Config not set")]
    UserConfigError,
    #[error("Path error")]
    PathError,
    #[error("Query Error")]
    QueryError(#[from] query_client::error::QueryError),
    
}