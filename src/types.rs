//! Common types used across the Search Console API.

use serde::{Deserialize, Serialize};

/// Filter group for dimension filters.
///
/// <https://developers.google.com/webmaster-tools/v1/searchanalytics/query#response>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionFilterGroup {
    /// How the filters are combined. Default is "and".
    #[serde(rename = "groupType")]
    pub group_type: Option<String>,
    /// List of filters in this group.
    pub filters: Vec<DimensionFilterGroupFilter>,
}

/// Individual filter within a dimension filter group.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionFilterGroupFilter {
    /// The dimension to filter on.
    pub dimension: Option<String>,
    /// The operator for the filter (e.g., "equals", "contains", "notContains").
    pub operator: Option<String>,
    /// The value to filter by.
    pub expression: Option<String>,
}

/// Dimension for grouping search analytics data.
///
/// Use dimensions to group your search analytics data by specific criteria.
///
/// # Example
///
/// ```rust
/// use google_search_console_api::types::Dimension;
///
/// let dimensions = vec![Dimension::Query, Dimension::Page, Dimension::Date];
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Dimension {
    /// Group by country (ISO 3166-1 alpha-3 country code).
    Country,
    /// Group by device type (desktop, mobile, tablet).
    Device,
    /// Group by page URL.
    Page,
    /// Group by search query.
    Query,
    /// Group by search appearance (e.g., rich results, AMP).
    SearchAppearance,
    /// Group by date.
    Date,
}

#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "Use `Dimension` instead")]
pub type DIMENSION = Dimension;

/// Sitemap information.
///
/// <https://developers.google.com/webmaster-tools/v1/sitemaps#resource>
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Sitemap {
    /// The URL of the sitemap.
    pub path: Option<String>,
    /// Date & time when this sitemap was last submitted.
    #[serde(rename = "lastSubmitted")]
    pub last_submitted: Option<String>,
    /// Whether the sitemap is pending processing.
    #[serde(rename = "isPending")]
    pub is_pending: Option<bool>,
    /// Whether this is a sitemap index file.
    #[serde(rename = "isSitemapsIndex")]
    pub is_sitemaps_index: Option<bool>,
    /// The type of sitemap (e.g., "sitemap", "rss", "atom", "urlList").
    #[serde(rename = "type")]
    pub sitemap_type: Option<String>,
    /// Date & time when this sitemap was last downloaded.
    #[serde(rename = "lastDownloaded")]
    pub last_downloaded: Option<String>,
    /// Number of warnings for this sitemap.
    pub warnings: Option<String>,
    /// Number of errors for this sitemap.
    pub errors: Option<String>,
    /// Contents of the sitemap.
    pub contents: Option<Vec<SitemapContent>>,
}

/// Content information within a sitemap.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SitemapContent {
    /// The type of content (e.g., "web", "image", "video", "news").
    #[serde(rename = "type")]
    pub content_type: Option<String>,
    /// Number of URLs submitted.
    pub submitted: Option<String>,
    /// Number of URLs indexed.
    pub indexed: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_serialize() {
        assert_eq!(
            serde_json::to_string(&Dimension::Country).unwrap(),
            "\"country\""
        );
        assert_eq!(
            serde_json::to_string(&Dimension::Device).unwrap(),
            "\"device\""
        );
        assert_eq!(
            serde_json::to_string(&Dimension::Page).unwrap(),
            "\"page\""
        );
        assert_eq!(
            serde_json::to_string(&Dimension::Query).unwrap(),
            "\"query\""
        );
        assert_eq!(
            serde_json::to_string(&Dimension::SearchAppearance).unwrap(),
            "\"searchAppearance\""
        );
        assert_eq!(
            serde_json::to_string(&Dimension::Date).unwrap(),
            "\"date\""
        );
    }

    #[test]
    fn test_dimension_deserialize() {
        assert_eq!(
            serde_json::from_str::<Dimension>("\"country\"").unwrap(),
            Dimension::Country
        );
        assert_eq!(
            serde_json::from_str::<Dimension>("\"query\"").unwrap(),
            Dimension::Query
        );
        assert_eq!(
            serde_json::from_str::<Dimension>("\"date\"").unwrap(),
            Dimension::Date
        );
    }

    #[test]
    fn test_dimension_equality() {
        assert_eq!(Dimension::Query, Dimension::Query);
        assert_ne!(Dimension::Query, Dimension::Page);
    }

    #[test]
    fn test_sitemap_deserialize() {
        let json = r#"{
            "path": "https://example.com/sitemap.xml",
            "lastSubmitted": "2024-01-01T00:00:00Z",
            "isPending": false,
            "isSitemapsIndex": false,
            "type": "sitemap"
        }"#;

        let sitemap: Sitemap = serde_json::from_str(json).unwrap();
        assert_eq!(
            sitemap.path,
            Some("https://example.com/sitemap.xml".to_string())
        );
        assert_eq!(sitemap.is_pending, Some(false));
        assert_eq!(sitemap.sitemap_type, Some("sitemap".to_string()));
    }

    #[test]
    fn test_sitemap_default() {
        let sitemap = Sitemap::default();
        assert!(sitemap.path.is_none());
        assert!(sitemap.is_pending.is_none());
    }

    #[test]
    fn test_dimension_filter_group_serialize() {
        let filter = DimensionFilterGroupFilter {
            dimension: Some("query".to_string()),
            operator: Some("contains".to_string()),
            expression: Some("test".to_string()),
        };

        let group = DimensionFilterGroup {
            group_type: Some("and".to_string()),
            filters: vec![filter],
        };

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("\"groupType\":\"and\""));
        assert!(json.contains("\"dimension\":\"query\""));
        assert!(json.contains("\"operator\":\"contains\""));
        assert!(json.contains("\"expression\":\"test\""));
    }
}
