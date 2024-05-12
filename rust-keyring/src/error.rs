use thiserror::Error;

#[derive(Error,Debug)]
pub enum FileCredentialError{
    #[error("Keyring Error")]
    KeyringError(#[from] keyring::Error),
    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Jose Error")]
    JoseError(#[from] josekit::JoseError),
    
    
}