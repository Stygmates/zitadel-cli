//! This module contains the environment variables for the zitadel CLI

use std::path::PathBuf;
/// Returns the path to the configuration file
/// Necessary for all flows
/// The configuration file is used to store the access token
/// The `CONFIG_FILE_PATH` environment variable must be an absolute path
pub(crate) fn config_file_path() -> PathBuf {
    std::env::var("CONFIG_FILE_PATH")
        .expect("CONFIG_FILE_PATH env variable not found")
        .parse()
        .expect("Failed to parse CONFIG_FILE_PATH")
}

/// Returns the issuer
/// Necessary for all flows
pub(crate) fn issuer() -> String {
    std::env::var("ISSUER").expect("ISSUER env variable not found")
}

/// Returns the client id
/// Necessary for `Authorization Code Flow` and `Client Credential Flow`
pub(crate) fn client_id() -> String {
    std::env::var("CLIENT_ID").expect("CLIENT_ID env variable not found")
}

/// Returns the client secret
/// Necessary for `Authorization Code Flow`
pub(crate) fn client_secret() -> String {
    std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET env variable not found")
}

/// Returns the callback server address
/// Necessary for `Authorization Code Flow`
pub(crate) fn callback_server_address() -> String {
    std::env::var("CALLBACK_SERVER_ADDRESS")
        .expect("CALLBACK_SERVER_ADDRESS env variable not found")
}

/// Returns the scopes
/// Necessary for all flows
pub(crate) fn scopes() -> String {
    std::env::var("SCOPES").expect("SCOPES env variable not found")
}

/// Returns whether the server is secure or not (https or http)
/// Necessary for `Authorization Code Flow`
pub(crate) fn is_secure() -> bool {
    std::env::var("IS_SECURE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false)
}

/// Returns the personal access token
/// Necessary for `Personal Access Token Flow`
pub(crate) fn personal_access_token() -> String {
    std::env::var("PERSONAL_ACCESS_TOKEN").expect("PERSONAL_ACCESS_TOKEN env variable not found")
}
