//! Search Analytics query types and builder.

use crate::types::{Dimension, DimensionFilterGroup};
use serde::{Deserialize, Serialize};

/// Search type for filtering results.
///
/// Specifies the type of search to query data for.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SearchType {
    /// Web search results (default).
    #[default]
    Web,
    /// Image search results.
    Image,
    /// Video search results.
    Video,
    /// News search results.
    News,
    /// Discover feed results.
    Discover,
    /// Google News app results.
    GoogleNews,
}

/// Aggregation type for grouping results.
///
/// Controls how data is aggregated in the response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum AggregationType {
    /// Auto aggregation based on dimensions (default).
    #[default]
    Auto,
    /// Aggregate by page URL.
    ByPage,
    /// Aggregate by property (site-level).
    ByProperty,
}

/// Data freshness state.
///
/// Controls whether to include preliminary or only finalized data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum DataState {
    /// Include all available data, including preliminary data (default).
    #[default]
    All,
    /// Only include finalized data.
    Final,
}

/// Request parameters for the Search Analytics query API.
///
/// Use the builder pattern for easier construction:
///
/// # Example
///
/// ```rust
/// use google_search_console_api::search_analytics::query::SearchAnalyticsQueryRequest;
/// use google_search_console_api::types::Dimension;
///
/// let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
///     .dimensions(vec![Dimension::Query, Dimension::Page])
///     .row_limit(1000)
///     .build();
/// ```
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryRequest {
    /// Start date of the query (YYYY-MM-DD format).
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// End date of the query (YYYY-MM-DD format).
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// Dimensions to group results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    /// Type of search to filter by.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub search_type: Option<SearchType>,
    /// Filter groups to apply to the query.
    #[serde(
        rename = "dimensionFilterGroups",
        skip_serializing_if = "Option::is_none"
    )]
    pub dimension_filter_groups: Option<Vec<DimensionFilterGroup>>,
    /// How to aggregate the results.
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<AggregationType>,
    /// Maximum number of rows to return (1-25000).
    #[serde(rename = "rowLimit", skip_serializing_if = "Option::is_none")]
    pub row_limit: Option<usize>,
    /// Zero-based index of the first row to return (for pagination).
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<usize>,
    /// Data freshness preference.
    #[serde(rename = "dataState", skip_serializing_if = "Option::is_none")]
    pub data_state: Option<DataState>,
}

impl SearchAnalyticsQueryRequest {
    /// Create a new builder for SearchAnalyticsQueryRequest.
    ///
    /// # Arguments
    ///
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_search_console_api::search_analytics::query::SearchAnalyticsQueryRequest;
    ///
    /// let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
    ///     .row_limit(100)
    ///     .build();
    /// ```
    pub fn builder(start_date: &str, end_date: &str) -> SearchAnalyticsQueryRequestBuilder {
        SearchAnalyticsQueryRequestBuilder {
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            ..Default::default()
        }
    }
}

/// Builder for [`SearchAnalyticsQueryRequest`].
///
/// Use [`SearchAnalyticsQueryRequest::builder`] to create an instance.
#[derive(Default, Debug, Clone)]
pub struct SearchAnalyticsQueryRequestBuilder {
    start_date: String,
    end_date: String,
    dimensions: Option<Vec<Dimension>>,
    search_type: Option<SearchType>,
    dimension_filter_groups: Option<Vec<DimensionFilterGroup>>,
    aggregation_type: Option<AggregationType>,
    row_limit: Option<usize>,
    start_row: Option<usize>,
    data_state: Option<DataState>,
}

impl SearchAnalyticsQueryRequestBuilder {
    /// Set dimensions for grouping results.
    ///
    /// Results will be grouped by the specified dimensions.
    pub fn dimensions(mut self, dimensions: Vec<Dimension>) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Set the search type filter.
    ///
    /// Filters results to only include data from the specified search type.
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }

    /// Set dimension filter groups.
    ///
    /// Filters results based on dimension values.
    pub fn dimension_filter_groups(mut self, groups: Vec<DimensionFilterGroup>) -> Self {
        self.dimension_filter_groups = Some(groups);
        self
    }

    /// Set the aggregation type.
    ///
    /// Controls how data is aggregated in the response.
    pub fn aggregation_type(mut self, aggregation_type: AggregationType) -> Self {
        self.aggregation_type = Some(aggregation_type);
        self
    }

    /// Set the maximum number of rows to return.
    ///
    /// Maximum value is 25000. Values greater than 25000 will be clamped.
    pub fn row_limit(mut self, limit: usize) -> Self {
        self.row_limit = Some(limit.min(25000));
        self
    }

    /// Set the starting row for pagination.
    ///
    /// Zero-based index of the first row to return.
    pub fn start_row(mut self, start: usize) -> Self {
        self.start_row = Some(start);
        self
    }

    /// Set the data state filter.
    ///
    /// Controls whether to include preliminary data.
    pub fn data_state(mut self, data_state: DataState) -> Self {
        self.data_state = Some(data_state);
        self
    }

    /// Build the request.
    ///
    /// Consumes the builder and returns the constructed request.
    pub fn build(self) -> SearchAnalyticsQueryRequest {
        SearchAnalyticsQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            dimensions: self.dimensions,
            search_type: self.search_type,
            dimension_filter_groups: self.dimension_filter_groups,
            aggregation_type: self.aggregation_type,
            row_limit: self.row_limit,
            start_row: self.start_row,
            data_state: self.data_state,
        }
    }
}

/// Response from a Search Analytics query.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryResponse {
    /// List of result rows.
    pub rows: Option<Vec<SearchAnalyticsQueryResponseRow>>,
    /// The aggregation type used for the response.
    #[serde(rename = "responseAggregationType")]
    pub response_aggregation_type: Option<String>,
}

/// A single row in the Search Analytics query response.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnalyticsQueryResponseRow {
    /// Values for the requested dimensions, in the order specified.
    pub keys: Option<Vec<String>>,
    /// Number of clicks.
    pub clicks: Option<f32>,
    /// Number of impressions.
    pub impressions: Option<f32>,
    /// Click-through rate (clicks / impressions).
    pub ctr: Option<f32>,
    /// Average position in search results.
    pub position: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31").build();

        assert_eq!(request.start_date, "2024-01-01");
        assert_eq!(request.end_date, "2024-01-31");
        assert!(request.dimensions.is_none());
        assert!(request.row_limit.is_none());
    }

    #[test]
    fn test_builder_with_dimensions() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
            .dimensions(vec![Dimension::Query, Dimension::Page])
            .build();

        let dims = request.dimensions.unwrap();
        assert_eq!(dims.len(), 2);
        assert_eq!(dims[0], Dimension::Query);
        assert_eq!(dims[1], Dimension::Page);
    }

    #[test]
    fn test_builder_row_limit_clamped() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
            .row_limit(50000)
            .build();

        assert_eq!(request.row_limit, Some(25000));
    }

    #[test]
    fn test_builder_row_limit_normal() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
            .row_limit(100)
            .build();

        assert_eq!(request.row_limit, Some(100));
    }

    #[test]
    fn test_builder_all_options() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
            .dimensions(vec![Dimension::Query])
            .search_type(SearchType::Web)
            .aggregation_type(AggregationType::ByPage)
            .data_state(DataState::Final)
            .row_limit(1000)
            .start_row(100)
            .build();

        assert_eq!(request.start_date, "2024-01-01");
        assert_eq!(request.end_date, "2024-01-31");
        assert_eq!(request.dimensions.unwrap().len(), 1);
        assert_eq!(request.search_type, Some(SearchType::Web));
        assert_eq!(request.aggregation_type, Some(AggregationType::ByPage));
        assert_eq!(request.data_state, Some(DataState::Final));
        assert_eq!(request.row_limit, Some(1000));
        assert_eq!(request.start_row, Some(100));
    }

    #[test]
    fn test_search_type_serialize() {
        assert_eq!(serde_json::to_string(&SearchType::Web).unwrap(), "\"web\"");
        assert_eq!(
            serde_json::to_string(&SearchType::Image).unwrap(),
            "\"image\""
        );
        assert_eq!(
            serde_json::to_string(&SearchType::Discover).unwrap(),
            "\"discover\""
        );
        assert_eq!(
            serde_json::to_string(&SearchType::GoogleNews).unwrap(),
            "\"googleNews\""
        );
    }

    #[test]
    fn test_aggregation_type_serialize() {
        assert_eq!(
            serde_json::to_string(&AggregationType::Auto).unwrap(),
            "\"auto\""
        );
        assert_eq!(
            serde_json::to_string(&AggregationType::ByPage).unwrap(),
            "\"byPage\""
        );
        assert_eq!(
            serde_json::to_string(&AggregationType::ByProperty).unwrap(),
            "\"byProperty\""
        );
    }

    #[test]
    fn test_data_state_serialize() {
        assert_eq!(serde_json::to_string(&DataState::All).unwrap(), "\"all\"");
        assert_eq!(
            serde_json::to_string(&DataState::Final).unwrap(),
            "\"final\""
        );
    }

    #[test]
    fn test_request_serialize() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
            .dimensions(vec![Dimension::Query])
            .row_limit(100)
            .build();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"startDate\":\"2024-01-01\""));
        assert!(json.contains("\"endDate\":\"2024-01-31\""));
        assert!(json.contains("\"dimensions\":[\"query\"]"));
        assert!(json.contains("\"rowLimit\":100"));
    }

    #[test]
    fn test_request_skip_none_fields() {
        let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31").build();

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("dimensions"));
        assert!(!json.contains("rowLimit"));
        assert!(!json.contains("type"));
    }

    #[test]
    fn test_response_deserialize() {
        let json = r#"{
            "rows": [
                {
                    "keys": ["test query"],
                    "clicks": 100.0,
                    "impressions": 1000.0,
                    "ctr": 0.1,
                    "position": 5.5
                }
            ],
            "responseAggregationType": "auto"
        }"#;

        let response: SearchAnalyticsQueryResponse = serde_json::from_str(json).unwrap();
        assert!(response.rows.is_some());

        let rows = response.rows.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].clicks, Some(100.0));
        assert_eq!(rows[0].impressions, Some(1000.0));
        assert_eq!(rows[0].ctr, Some(0.1));
        assert_eq!(rows[0].position, Some(5.5));
    }

    #[test]
    fn test_response_empty_rows() {
        let json = r#"{}"#;

        let response: SearchAnalyticsQueryResponse = serde_json::from_str(json).unwrap();
        assert!(response.rows.is_none());
    }
}
