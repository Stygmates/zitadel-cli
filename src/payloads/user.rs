use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewHumanUser {
    user_id: Option<String>,
    username: Option<String>,
    organization: Option<Organization>,
    profile: Profile,
    email: Email,
    phone: Option<Phone>,
    metadata: Option<Vec<Metadata>>,
    password: Option<Password>,
    hashed_password: Option<HashedPassword>,
    idp_links: Option<Vec<IDPLink>>,
    totp_secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Human {
    user_id: Option<String>,
    username: Option<String>,
    organization: Option<Organization>,
    profile: Profile,
    email: Email,
    phone: Option<Phone>,
    metadata: Option<Vec<Metadata>>,
    password: Option<Password>,
    hashed_password: Option<HashedPassword>,
    idp_links: Option<Vec<IDPLink>>,
    totp_secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    org_id: Option<String>,
    org_domain: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    given_name: String,
    family_name: String,
    nick_name: Option<String>,
    display_name: Option<String>,
    preferred_language: Option<String>,
    gender: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    email: String,
    send_code: Option<SendCode>,
    return_code: Option<ReturnCode>,
    is_verified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendCode {
    url_template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ReturnCode {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    phone: Option<String>,
    send_code: Option<SendCode>,
    return_code: Option<ReturnCode>,
    is_verified: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Password {
    password: String,
    change_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HashedPassword {
    hash: String,
    change_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IDPLink {
    idp_id: Option<String>,
    user_id: Option<String>,
    user_name: Option<String>,
}
