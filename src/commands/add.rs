use reqwest::{
    header::{HeaderValue, LOCATION},
    Client, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use tracing::{error, info};

use crate::{error::ZitadelCLIError, payloads::load_from_file};

use super::{load_access_token, Token};

/// Generic function to add an entity to the Zitadel instance
pub(crate) async fn handle_add_entity<T>(file_path: &Path, endpoint: &str)
where
    T: serde::de::DeserializeOwned + Serialize,
{
    match load_access_token() {
        Ok(token) => match add_entity::<T>(token, endpoint, file_path).await {
            Ok(location) => match location {
                Some(location) => info!(
                    "{} added successfully at {}",
                    std::any::type_name::<T>(),
                    location.to_str().unwrap()
                ),
                None => info!("{} added successfully", std::any::type_name::<T>()),
            },
            Err(error) => error!("Error adding {}: {}", std::any::type_name::<T>(), error),
        },
        Err(error) => error!("Error loading token: {}", error),
    }
}

pub(crate) async fn add_entity<T: DeserializeOwned + Serialize>(
    token: Token,
    endpoint: &str,
    entity_file_path: &Path,
) -> Result<Option<HeaderValue>, ZitadelCLIError> {
    let issuer = std::env::var("ISSUER").expect("ISSUER env variable not found");
    match load_from_file::<T>(entity_file_path) {
        Ok(entity) => {
            match add_entity_api_call(&token.access_token, &issuer, endpoint, entity).await {
                Ok(response) => match response.status() {
                    StatusCode::CREATED | StatusCode::OK => {
                        Ok(response.headers().get(LOCATION).cloned())
                    }
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
