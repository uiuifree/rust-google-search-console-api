use serde::{Deserialize, Serialize};
use crate::types::{DIMENSION, DimensionFilterGroup};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryRequest {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub dimensions: Option<Vec<DIMENSION>>,
    #[serde(rename = "search_type")]
    pub search_type: Option<String>,
    #[serde(rename = "type")]
    pub query_type: Option<String>,
    #[serde(rename = "dimensionFilterGroups")]
    pub dimension_filter_groups: Option<Vec<DimensionFilterGroup>>,
    #[serde(rename = "aggregationType")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "rowLimit")]
    pub row_limit: Option<usize>,
    #[serde(rename = "startRow")]
    pub start_row: Option<usize>,
    #[serde(rename = "dataState")]
    pub data_state: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryResponse {
    pub rows: Option<Vec<SearchAnalyticsQueryResponseRow>>,
    #[serde(rename = "responseAggregationType")]
    pub response_aggregation_type: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryResponseRow {
    pub keys: Option<Vec<String>>,
    pub clicks: Option<f32>,
    pub impressions: Option<f32>,
    pub ctr: Option<f32>,
    pub position: Option<f32>,
}
