/*
Define json response/request schemas
 */
use serde::{Deserialize, Serialize};

/*
GET /api/account
 */
// Nested struct helper
#[derive(Serialize, Deserialize, Debug)]
pub struct Nest<T> {
    #[serde(flatten)]
    inner: T,
}

// Define AccessToken struct
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    #[serde(rename = "displayName")]
    display_name: String,
    role: String,
}
// Define Token struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    #[serde(rename = "type")]
    auth_type: String,
    #[serde(rename = "accessToken")]
    access_token: Nest<AccessToken>,
}
// Define response struct
#[derive(Serialize, Deserialize, Debug)]
pub struct AcctResponse {
    #[serde(rename = "type")]
    acct_type: String,
    id: String,
    name: String,
    fullname: String,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    plan: String,
    #[serde(rename = "canPay")]
    can_pay: bool,
    #[serde(rename = "isPro")]
    is_pro: bool,
    #[serde(rename = "periodEnd")]
    period_end: Option<String>,
    #[serde(rename = "avatarUrl")]
    avatar_url: String,
    orgs: Vec<String>,
    auth: Nest<Auth>,
}

/*
POST /api/repo
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NewRepo {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub name: String,
    pub private: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoResponse {
    pub url: String,
}

/*
DELETE /api/repo
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteRepo {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub name: String,
}

/*
PUT /api/repo
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRepo {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub namespace: String,
    pub private: bool,
}
