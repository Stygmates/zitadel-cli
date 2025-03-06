use std::path::PathBuf;

use serde::Serialize;

use crate::{
    env::{config_file_path, personal_access_token},
    error::ZitadelCLIError,
};

use super::save_config;

#[derive(Debug, Serialize)]
/// The configuration for the personal access token flow
struct PersonalAccessTokenFlowAppConfig {
    config_file_path: PathBuf,
    personal_access_token: String,
}

/// Logs the user in using the personal access token flow
/// Writes the personal access token to the config file
pub(crate) async fn login() -> Result<(), ZitadelCLIError> {
    let config: PersonalAccessTokenFlowAppConfig = init_config_from_env();
    save_config(&config.config_file_path, config.personal_access_token)
}

/// Initializes the configuration for the personal access token flow
fn init_config_from_env() -> PersonalAccessTokenFlowAppConfig {
    let config_file_path = config_file_path();
    let personal_access_token = personal_access_token();
    PersonalAccessTokenFlowAppConfig {
        config_file_path,
        personal_access_token,
    }
}
