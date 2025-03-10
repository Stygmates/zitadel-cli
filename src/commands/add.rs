use std::path::Path;

use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::ZitadelCLIError, payloads::load_from_file};

use super::Token;

pub(crate) async fn add_entity<T: DeserializeOwned + Serialize>(
    token: Token,
    endpoint: &str,
    entity_file_path: &Path,
) -> Result<(), ZitadelCLIError> {
    let issuer = std::env::var("ISSUER").expect("ISSUER env variable not found");
    match load_from_file::<T>(entity_file_path) {
        Ok(entity) => {
            match add_entity_api_call(&token.access_token, &issuer, endpoint, entity).await {
                Ok(response) => match response.status() {
                    StatusCode::CREATED | StatusCode::OK => Ok(()),
                    StatusCode::UNAUTHORIZED => Err(ZitadelCLIError::ReqwestResponse(format!(
                        "Invalid token or unauthorized access, please log in again: {}",
                        response.text().await?,
                    ))),
                    status_code => Err(ZitadelCLIError::ReqwestResponse(format!(
                        "Unhandled status code {status_code}: {}",
                        response.text().await?,
                    ))),
                },
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

async fn add_entity_api_call<T: Serialize>(
    access_token: &str,
    issuer: &str,
    endpoint: &str,
    entity: T,
) -> Result<Response, ZitadelCLIError> {
    let client = Client::new();
    let request = client
        .post(format!("{issuer}{endpoint}"))
        .header("Authorization", format! {"Bearer {access_token}"})
        .json(&entity)
        .send()
        .await?;
    Ok(request)
}
