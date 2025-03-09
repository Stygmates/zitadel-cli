use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// A Project is a vessel for different applications sharing the same role context.
/// For more details about each field, you can look at the [documentation](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project)
#[serde(rename_all = "camelCase")]
pub(crate) struct NewProject {
    name: String,
    project_role_assertion: Option<bool>,
    project_role_check: Option<bool>,
    has_project_check: Option<bool>,
    private_labeling_setting: Option<String>,
}
