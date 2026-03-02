//! Sites API types.

use serde_derive::{Deserialize, Serialize};

/// Response from the Sites get API.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSiteApi {
    /// The URL of the site.
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    /// The user's permission level for this site.
    #[serde(rename = "permissionLevel")]
    pub permission_level: PermissionLevel,
}

/// Response from the Sites list API.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSiteListApi {
    /// List of sites the user has access to.
    #[serde(rename = "siteEntry", default)]
    pub site_entry: Vec<ResponseSiteApi>,
}

/// Permission level for a site.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub enum PermissionLevel {
    /// Full user permissions.
    #[serde(rename = "siteFullUser")]
    SiteFullUser,
    /// Owner permissions.
    #[serde(rename = "siteOwner")]
    SiteOwner,
    /// Restricted user permissions.
    #[serde(rename = "siteRestrictedUser")]
    SiteRestrictedUser,
    /// Unverified user (default).
    #[serde(rename = "siteUnverifiedUser")]
    #[default]
    SiteUnverifiedUser,
}
