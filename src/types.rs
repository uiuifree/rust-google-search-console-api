use serde::{Deserialize, Serialize};

/// https://developers.google.com/webmaster-tools/v1/searchanalytics/query#response
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionFilterGroup {
    #[serde(rename = "groupType")]
    pub group_type: Option<String>,
    pub filters: Vec<DimensionFilterGroupFilter>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionFilterGroupFilter {
    pub dimension: Option<String>,
    pub operator: Option<String>,
    pub expression: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DIMENSION {
    // #[serde(rename = "MATCH_TYPE_UNSPECIFIED")]
    Country(String),
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "searchAppearance")]
    SearchAppearance,
}

/// https://developers.google.com/webmaster-tools/v1/sitemaps#resource
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Sitemap {
    pub path: Option<String>,
    #[serde(rename = "lastSubmitted")]
    pub last_submitted: Option<String>,
    #[serde(rename = "isPending")]
    pub is_pending: Option<bool>,
    #[serde(rename = "isSitemapsIndex")]
    pub is_sitemaps_index: Option<bool>,
    #[serde(rename = "type")]
    pub sitemap_type: Option<String>,
    #[serde(rename = "lastDownloaded")]
    pub last_downloaded: Option<String>,
    pub warnings: Option<String>,
    pub errors: Option<String>,
    pub contents: Option<Vec<SitemapContent>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SitemapContent {
    #[serde(rename = "type")]
    pub content_type: Option<String>,
    pub submitted: Option<String>,
    pub indexed: Option<String>,
}