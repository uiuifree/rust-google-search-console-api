use serde_derive::{Deserialize, Serialize};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSiteApi {
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    #[serde(rename = "permissionLevel")]
    pub permission_level: PermissionLevel,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSiteListApi {
    #[serde(rename = "siteEntry")]
    pub site_entry: Vec<ResponseSiteApi>,
}


#[derive( Default,Debug, Serialize, Deserialize, Clone)]
pub enum PermissionLevel {
    #[serde(rename = "siteFullUser")]
    SiteFullUser,
    #[serde(rename = "siteOwner")]
    SiteOwner,
    #[serde(rename = "siteRestrictedUser")]
    SiteRestrictedUser,
    #[serde(rename = "siteUnverifiedUser")]
    #[default]
    SiteUnverifiedUser,
}