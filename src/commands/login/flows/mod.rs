use std::path::PathBuf;

use crate::error::ZitadelCLIError;
use authorization::login as authorization_login;
use clap::ValueEnum;
use client_credential::login as client_credential_login;
use personal_access_token::login as personal_access_token_login;
use serde::Serialize;
use tracing::{error, info};

pub(crate) mod authorization;
pub(crate) mod client_credential;
pub(crate) mod personal_access_token;

#[derive(Debug, Clone, ValueEnum)]
pub enum Flow {
    AuthorizationCode,
    ClientCredentials,
    PersonalAccessToken,
}

impl Flow {
    /// Logs the user in using the specified flow
    /// Writes the access token to the config file
    pub async fn login(self, open_browser: bool) -> Result<(), ZitadelCLIError> {
        match self {
            Flow::AuthorizationCode => authorization_login(open_browser).await,
            Flow::ClientCredentials => client_credential_login().await,
            Flow::PersonalAccessToken => personal_access_token_login().await,
        }
    }
}

/// Writes the access token to the config file
/// - `file_path`: The path to the config file
/// - `access_token`: The access token to write to the config file
/// - Returns `Ok(())` if the access token was written successfully
/// - Returns `Err(ZitadelCLIError::IO(error))` if the access token was not written successfully
pub fn save_config<T: Serialize>(
    file_path: &PathBuf,
    access_token: T,
) -> Result<(), ZitadelCLIError> {
    info! {"Writing the access token to a file"};
    match std::fs::write(file_path, serde_json::to_string(&access_token)?) {
        Ok(_) => {
            info! {"Access token successfully written to file! You can now use other commands"};
            Ok(())
        }
        Err(error) => {
            error!("Error writing access token to file: {error}");
            Err(ZitadelCLIError::IO(error))
        }
    }
}
