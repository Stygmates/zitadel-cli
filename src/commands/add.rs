use std::path::Path;

use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::ZitadelCLIError, payloads::load_from_file};

use super::Token;

pub(crate) async fn add_ressource<T: DeserializeOwned + Serialize>(
    token: Token,
    endpoint: &str,
    ressource_file_path: &Path,
) -> Result<(), ZitadelCLIError> {
    let issuer = std::env::var("ISSUER").expect("ISSUER env variable not found");
    match load_from_file::<T>(ressource_file_path) {
        Ok(ressource) => {
            match add_ressource_api_call(&token.access_token, &issuer, endpoint, ressource).await {
                Ok(response) => match response.status() {
                    StatusCode::CREATED => Ok(()),
                    StatusCode::UNAUTHORIZED => Err(ZitadelCLIError::ReqwestResponse(format!(
                        "Invalid token or unauthorized access, please log in again: {}",
                        response.text().await?,
                    ))),
                    _ => Err(ZitadelCLIError::ReqwestResponse(format!(
                        "An unexpected error occured: {}",
                        response.text().await?,
                    ))),
                },
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

async fn add_ressource_api_call<T: Serialize>(
    access_token: &str,
    issuer: &str,
    endpoint: &str,
    ressource: T,
) -> Result<Response, ZitadelCLIError> {
    let client = Client::new();
    let request = client
        .post(format!("{issuer}{endpoint}"))
        .header("Authorization", format! {"Bearer {access_token}"})
        .json(&ressource)
        .send()
        .await?;
    Ok(request)
}
