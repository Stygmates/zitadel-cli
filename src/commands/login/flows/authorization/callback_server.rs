//! This module starts a server that listens for the callback from the `authorization endpoint` and then exchanges the code for an access token
//! The access token is then written to a file and the server is stopped
//! The server is started by the `login` command

use std::sync::Mutex;
use tracing::error;

use actix_web::{
    dev::{Server, ServerHandle},
    get, web, App, HttpServer, Responder,
};
use serde::Deserialize;

use crate::{
    commands::login::flows::{authorization::exchange_code, save_config},
    env::{callback_server_address, client_id, config_file_path, is_secure, issuer, scopes},
    error::ZitadelCLIError,
};

use crate::commands::{discover, DiscoveryConfig};

use super::{generate_code_challenge, generate_code_verifier, AuthorizationFlowAppConfig};

#[derive(Deserialize)]
struct QueryParams {
    code: String,
}

#[get("/callback")]
/// Callback used by the `authorization endpoint` to return the `code`, also writes the token to the file specified in the `CONFIG_FILE_PATH` environment variable
/// and stops the server once the token retrieved and written to the file specified in the `CONFIG_FILE_PATH` environment variable
/// - `query` is the query parameters from the `authorization endpoint`
/// - `config` is the configuration of the application
/// - `discovery_config` is the configuration of the OpenID Connect provider
/// - `stop_handle` is the handle to stop the server
async fn callback(
    query: web::Query<QueryParams>,
    config: web::Data<tokio::sync::Mutex<AuthorizationFlowAppConfig>>,
    discovery_config: web::Data<Mutex<DiscoveryConfig>>,
    stop_handle: web::Data<StopHandle>,
) -> impl Responder {
    let code = query.code.clone();
    let config = config.clone();
    let discovery_config: web::Data<Mutex<DiscoveryConfig>> = discovery_config.clone();

    let result = async {
        let config_val = config.lock().await;
        let redirect_uri = config_val.redirect_uri();
        let client_id = config_val.client_id.clone();
        let code_verifier = config_val.code_verifier.clone();
        let code_challenge = config_val.code_challenge.clone();
        let token_endpoint = discovery_config.lock().unwrap().token_endpoint.clone();
        match exchange_code(
            token_endpoint,
            client_id,
            code,
            redirect_uri.clone(),
            code_verifier,
            code_challenge,
        )
        .await
        {
            Ok(code_response) => save_config(&config_val.config_file_path, code_response),
            Err(error) => {
                error! {"Error exchanging code: {error}"};
                Err(error)
            }
        }
    }
    .await;

    match result {
        Ok(_) => {
            stop_handle.stop(true);
            format!("Code received and access_token retrieved, you can close this page.")
        }
        Err(err) => {
            error!("An error occured: {}", err);
            format!("An error occurred: {}", err)
        }
    }
}

/// Initializes the configuration from the environment variables, here is the full list:
/// - `ISSUER` The url of the Zitadel instance
/// - `CLIENT_ID` is the client id of the application
/// - `CALLBACK_SERVER_ADDRESS` is the address of the server that listens for the callback
/// - `SCOPES` is the scopes to request from the OpenID Connect provider
/// - `IS_SECURE` is a boolean that indicates if the server is secure
/// Returns the configuration of the application
pub(crate) fn init_config_from_env() -> Result<AuthorizationFlowAppConfig, ZitadelCLIError> {
    let config_file_path = config_file_path();
    let issuer = issuer();
    let client_id = client_id();
    let callback_server_address = callback_server_address();
    let scopes = scopes();
    let is_secure = is_secure();
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(code_verifier.clone());

    Ok(AuthorizationFlowAppConfig {
        config_file_path,
        issuer,
        client_id,
        callback_server_address,
        is_secure,
        scopes,
        code_verifier,
        code_challenge,
    })
}

/// Initializes the server that listens for the callback from the `authorization endpoint`
/// - `config` is the configuration of the application
pub(crate) async fn init_callback_server(
    config: AuthorizationFlowAppConfig,
) -> Result<Server, std::io::Error> {
    let callback_server_address = config.callback_server_address.clone();
    let openid_config = discover(&format! {"{}/.well-known/openid-configuration",&config.issuer})
        .await
        .unwrap();

    // Tokio mutex needed because we modify the value of the config in an async operation
    let config_handle = web::Data::new(tokio::sync::Mutex::new(config));

    let openid_config_handle: web::Data<Mutex<DiscoveryConfig>> =
        web::Data::new(Mutex::new(openid_config));

    let stop_handle = web::Data::new(StopHandle::default());
    let server = HttpServer::new({
        let stop_handle = stop_handle.clone();
        move || {
            App::new()
                .service(callback)
                .app_data(web::Data::clone(&openid_config_handle))
                .app_data(web::Data::clone(&config_handle))
                .app_data(web::Data::clone(&stop_handle.clone()))
        }
    })
    .bind(callback_server_address)?
    .run();

    // register the server handle with the stop handle
    stop_handle.register(server.handle());

    Ok(server)
}

#[derive(Default)]
/// Struct to handle stopping the server
struct StopHandle {
    inner: parking_lot::Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        #[allow(clippy::let_underscore_future)]
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}
