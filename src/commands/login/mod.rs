pub(crate) mod flows;

use serde::Deserialize;

use crate::{env::config_file_path, error::ZitadelCLIError};

/// NOTE: Some of the fields are missing right now and can be added later on as needed
/// See [the openID Connect documentation](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata)
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DiscoveryConfig {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub introspection_endpoint: String,
    pub revocation_endpoint: String,
    pub end_session_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub request_object_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub revocation_endpoint_auth_methods_supported: Vec<String>,
    pub revocation_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub introspection_endpoint_auth_methods_supported: Vec<String>,
    pub introspection_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub ui_locales_supported: Vec<String>,
    pub request_parameter_supported: bool,
    pub request_uri_parameter_supported: bool,
}

/// Calls the `/.well-known/openid-configuration` of the provided url
pub async fn discover(url: &str) -> Result<DiscoveryConfig, ZitadelCLIError> {
    Ok(reqwest::get(url).await?.json::<DiscoveryConfig>().await?)
}

#[derive(Deserialize)]
pub(crate) struct Token {
    pub access_token: String,
}

pub(crate) fn load_access_token() -> Result<Token, ZitadelCLIError> {
    let token_str = std::fs::read_to_string(config_file_path())?;
    Ok(serde_json::from_str(&token_str)?)
}
