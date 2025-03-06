use std::{collections::HashMap, path::PathBuf};

use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    commands::discover,
    env::{client_id, client_secret, config_file_path, issuer, scopes},
    error::ZitadelCLIError,
};

use super::save_config;

struct ClientCredentialFlowAppConfig {
    config_file_path: PathBuf,
    client_id: String,
    client_secret: String,
    scopes: String,
    issuer: String,
}

#[derive(Debug, serde::Deserialize, Serialize)]
struct ClientCredentialsResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

/// Logs the user in using the client credential flow
/// Writes the access token to the config file
pub(crate) async fn login() -> Result<(), ZitadelCLIError> {
    let config: ClientCredentialFlowAppConfig = init_config_from_env();
    let discover_config =
        discover(&format! {"{}/.well-known/openid-configuration", config.issuer}).await?;
    let url = discover_config.token_endpoint;

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "client_credentials");
    form_data.insert("client_id", &config.client_id);
    form_data.insert("client_secret", &config.client_secret);
    form_data.insert("scope", &config.scopes);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .form(&form_data)
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .send()
        .await?;
    match response.status() {
        StatusCode::OK => {
            let parsed_response = response.json::<ClientCredentialsResponse>().await?;
            save_config(&config.config_file_path, parsed_response)
        }
        _ => {
            return Err(ZitadelCLIError::ReqwestResponse(format!(
                "An unexpected error occured: {}",
                response.text().await?,
            )))
        }
    }
}

/// Initializes the configuration from the environment variables, here is the full list:
/// - `DISCOVERY_ENDPOINT` is the endpoint to discover the OpenID Connect provider
/// - `CLIENT_ID` is the client id of the application
/// - `CALLBACK_SERVER_ADDRESS` is the address of the server that listens for the callback
/// - `SCOPES` is the scopes to request from the OpenID Connect provider
/// - `IS_SECURE` is a boolean that indicates if the server is secure
/// Returns the configuration of the application
fn init_config_from_env() -> ClientCredentialFlowAppConfig {
    let config_file_path = config_file_path();
    let issuer = issuer();
    let client_id: String = client_id();
    let client_secret = client_secret();
    let scopes = scopes();

    ClientCredentialFlowAppConfig {
        config_file_path,
        issuer,
        client_id,
        scopes,
        client_secret,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_init_config_from_env() {
        std::env::set_var("CONFIG_FILE_PATH", "/tmp/config.json");
        std::env::set_var("ISSUER", "https://issuer.com");
        std::env::set_var("CLIENT_ID", "client_id");
        std::env::set_var("CLIENT_SECRET", "client_secret");
        std::env::set_var("SCOPES", "openid profile email");

        let config = super::init_config_from_env();
        assert_eq!(config.config_file_path, Path::new("/tmp/config.json"));
        assert_eq!(config.issuer, "https://issuer.com");
        assert_eq!(config.client_id, "client_id");
        assert_eq!(config.client_secret, "client_secret");
        assert_eq!(config.scopes, "openid profile email");
    }
}
