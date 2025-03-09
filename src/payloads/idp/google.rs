//! This module contains the structure to add a google identity provider
//! For more infos see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-google-provider)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The structure of the google idp to be created
pub(crate) struct NewGoogleIdp {
    name: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    scopes: Option<Vec<String>>,
    provider_options: Option<NewGoogleIdpProviderOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The structure of the google idp options to be created
pub(crate) struct NewGoogleIdpProviderOptions {
    is_linking_allowed: Option<bool>,
    is_creation_allowed: Option<bool>,
    is_auto_creation: Option<bool>,
    is_auto_update: Option<bool>,
    auto_linking: Option<String>,
}
