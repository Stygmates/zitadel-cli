//! This module contains the structures for the organization service
//! For more infos see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization)

use serde::{Deserialize, Serialize};

use super::user::Human;

#[derive(Serialize, Deserialize, Debug)]
/// The structure of the organization to be created
pub(crate) struct NewOrganization {
    pub(crate) name: String,
    pub(crate) admins: Option<Vec<Admin>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Admin {
    user_id: Option<String>,
    human: Option<Human>,
    roles: Option<Vec<String>>,
}
