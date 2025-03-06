//! Error handling for the Zitadel CLI

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZitadelCLIError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalid response: {0}")]
    ReqwestResponse(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse url: {0}")]
    URLParse(#[from] url::ParseError),
    #[error("Failed to parse json:  {0}")]
    JSONParse(#[from] serde_json::Error),
}
