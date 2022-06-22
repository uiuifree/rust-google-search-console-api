use serde::{Deserialize, Serialize};

/// https://developers.google.com/webmaster-tools/v1/searchanalytics/query#response
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionFilterGroup {
    #[serde(rename = "groupType")]
    pub group_type: Option<String>,
    pub filters: Option<String>,
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
