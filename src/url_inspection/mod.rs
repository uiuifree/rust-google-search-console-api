//! URL Inspection API types.
//!
//! Types for inspecting URLs and getting indexing status, crawl info, and more.

use serde_derive::{Deserialize, Serialize};

/// Request parameters for URL inspection.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestUrlInspection {
    /// The fully-qualified URL to inspect.
    #[serde(rename = "inspectionUrl")]
    pub inspection_url: String,
    /// The URL of the property as defined in Search Console.
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    /// Language code for localized results (e.g., "en", "ja").
    #[serde(rename = "languageCode")]
    pub language_code: String,
}

/// Response from the URL inspection API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseInspectionResult {
    /// The inspection result.
    #[serde(rename = "inspectionResult")]
    pub inspection_result: InspectionResult,
}

/// Complete inspection result for a URL.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InspectionResult {
    /// Link to the full inspection result in Search Console.
    #[serde(rename = "inspectionResultLink")]
    pub inspection_result_link: Option<String>,
    /// Index status information.
    #[serde(rename = "indexStatusResult")]
    pub index_status_result: Option<IndexStatusResult>,
    /// AMP inspection result.
    #[serde(rename = "ampResult")]
    pub amp_result: Option<AmpInspectionResult>,
    /// Mobile usability inspection result.
    #[serde(rename = "mobileUsabilityResult")]
    pub mobile_usability_result: Option<MobileUsabilityInspectionResult>,
    /// Rich results inspection result.
    #[serde(rename = "richResultsResult")]
    pub rich_results_result: Option<RichResultsInspectionResult>,
}

/// Index status information for a URL.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexStatusResult {
    /// Sitemaps that contain this URL.
    pub sitemap: Option<Vec<String>>,
    /// URLs that refer to this URL.
    #[serde(rename = "referringUrls")]
    pub referring_urls: Option<Vec<String>>,
    /// Overall verdict for indexing.
    pub verdict: Option<Verdict>,
    /// Coverage state description.
    #[serde(rename = "coverageState")]
    pub coverage_state: Option<String>,
    /// Whether robots.txt allows crawling.
    #[serde(rename = "robotsTxtState")]
    pub robots_txt_state: Option<RobotsTxtState>,
    /// Whether indexing is allowed.
    #[serde(rename = "indexingState")]
    pub indexing_state: Option<IndexingState>,
    /// Last crawl time.
    #[serde(rename = "lastCrawlTime")]
    pub last_crawl_time: Option<String>,
    /// Page fetch state.
    #[serde(rename = "pageFetchState")]
    pub page_fetch_state: Option<PageFetchState>,
    /// Google-selected canonical URL.
    #[serde(rename = "googleCanonical")]
    pub google_canonical: Option<String>,
    /// User-declared canonical URL.
    #[serde(rename = "userCanonical")]
    pub user_canonical: Option<String>,
    /// User agent used for crawling.
    #[serde(rename = "crawledAs")]
    pub crawled_as: Option<CrawlUserAgent>,
}

/// Overall verdict for an inspection.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Verdict {
    /// Unspecified verdict.
    #[serde(rename = "VERDICT_UNSPECIFIED")]
    VerdictUnspecified,
    /// Passed inspection.
    PASS,
    /// Partially passed inspection.
    PARTIAL,
    /// Failed inspection.
    FAIL,
    /// Neutral result.
    NEUTRAL,
}

/// Robots.txt crawl permission state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RobotsTxtState {
    /// Unspecified state.
    #[serde(rename = "ROBOTS_TXT_STATE_UNSPECIFIED")]
    RobotsTxtStateUnspecified,
    /// Crawling is allowed by robots.txt.
    ALLOWED,
    /// Crawling is disallowed by robots.txt.
    DISALLOWED,
}

/// Indexing permission state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IndexingState {
    /// Unspecified state.
    #[serde(rename = "INDEXING_STATE_UNSPECIFIED")]
    IndexingStateUnspecified,
    /// Indexing is allowed.
    #[serde(rename = "INDEXING_ALLOWED")]
    IndexingAllowed,
    /// Blocked by meta tag.
    #[serde(rename = "BLOCKED_BY_META_TAG")]
    BlockedByMetaTag,
    /// Blocked by HTTP header.
    #[serde(rename = "BLOCKED_BY_HTTP_HEADER")]
    BlockedByHttpHeader,
    /// Blocked by robots.txt.
    #[serde(rename = "BLOCKED_BY_ROBOTS_TXT")]
    BlockedByRobotsTxt,
}

/// Page fetch state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PageFetchState {
    /// Unspecified state.
    #[serde(rename = "PAGE_FETCH_STATE_UNSPECIFIED")]
    PageFetchStateUnspecified,
    /// Page fetched successfully.
    SUCCESSFUL,
    /// Soft 404 detected.
    #[serde(rename = "SOFT_404")]
    Soft404,
    /// Blocked by robots.txt.
    #[serde(rename = "BLOCKED_ROBOTS_TXT")]
    BlockedRobotsTxt,
    /// Page not found (404).
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    /// Access denied (401).
    #[serde(rename = "ACCESS_DENIED")]
    AccessDenied,
    /// Server error (5xx).
    #[serde(rename = "SERVER_ERROR")]
    ServerError,
    /// Redirect error.
    #[serde(rename = "REDIRECT_ERROR")]
    RedirectError,
    /// Access forbidden (403).
    #[serde(rename = "ACCESS_FORBIDDEN")]
    AccessForbidden,
    /// Blocked by 4xx error.
    #[serde(rename = "BLOCKED_4XX")]
    Blocked4xx,
    /// Internal crawl error.
    #[serde(rename = "INTERNAL_CRAWL_ERROR")]
    InternalCrawlError,
    /// Invalid URL.
    #[serde(rename = "INVALID_URL")]
    InvalidUrl,
}

/// User agent used for crawling.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrawlUserAgent {
    /// Unspecified user agent.
    #[serde(rename = "CRAWLING_USER_AGENT_UNSPECIFIED")]
    CrawlingUserAgentUnspecified,
    /// Desktop user agent.
    DESKTOP,
    /// Mobile user agent.
    MOBILE,
}

/// AMP inspection result.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmpInspectionResult {
    /// List of AMP issues.
    pub issues: Option<Vec<AmpIssue>>,
    /// Overall AMP verdict.
    pub verdict: Option<Verdict>,
    /// The AMP URL.
    #[serde(rename = "ampUrl")]
    pub amp_url: Option<String>,
    /// Robots.txt state for AMP.
    #[serde(rename = "robotsTxtState")]
    pub robots_txt_state: Option<RobotsTxtState>,
    /// AMP indexing state.
    #[serde(rename = "indexingState")]
    pub indexing_state: Option<AmpIndexingState>,
    /// AMP index status verdict.
    #[serde(rename = "ampIndexStatusVerdict")]
    pub amp_index_status_verdict: Option<Verdict>,
    /// Last crawl time for AMP.
    #[serde(rename = "lastCrawlTime")]
    pub last_crawl_time: Option<String>,
    /// Page fetch state for AMP.
    #[serde(rename = "pageFetchState")]
    pub page_fetch_state: Option<PageFetchState>,
}

/// AMP indexing state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AmpIndexingState {
    /// Unspecified state.
    #[serde(rename = "AMP_INDEXING_STATE_UNSPECIFIED")]
    AmpIndexingStateUnspecified,
    /// AMP indexing is allowed.
    #[serde(rename = "AMP_INDEXING_ALLOWED")]
    AmpIndexingAllowed,
    /// Blocked due to noindex.
    #[serde(rename = "BLOCKED_DUE_TO_NOINDEX")]
    BlockedDueToNoindex,
    /// Blocked due to expired unavailable_after.
    #[serde(rename = "BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER")]
    BlockedDueToExpiredUnavailableAfter,
}

/// AMP issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmpIssue {
    /// Issue message.
    #[serde(rename = "issueMessage")]
    pub issue_message: Option<String>,
    /// Issue severity.
    pub severity: Option<Severity>,
}

/// Issue severity level.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    /// Unspecified severity.
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    SeverityUnspecified,
    /// Warning level.
    WARNING,
    /// Error level.
    ERROR,
}

/// Mobile usability inspection result.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileUsabilityInspectionResult {
    /// List of mobile usability issues.
    pub issues: Option<Vec<MobileUsabilityIssue>>,
    /// Overall mobile usability verdict.
    pub verdict: Option<Verdict>,
}

/// Mobile usability issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileUsabilityIssue {
    /// Type of mobile usability issue.
    #[serde(rename = "issueType")]
    pub issue_type: Option<MobileUsabilityIssueType>,
    /// Issue severity.
    pub severity: Option<Severity>,
    /// Issue message.
    pub message: Option<String>,
}

/// Type of mobile usability issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileUsabilityIssueType {
    /// Unspecified issue type.
    #[serde(rename = "MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED")]
    MobileUsabilityIssueTypeUnspecified,
    /// Uses incompatible plugins (e.g., Flash).
    #[serde(rename = "USES_INCOMPATIBLE_PLUGINS")]
    UsesIncompatiblePlugins,
    /// Viewport not configured.
    #[serde(rename = "CONFIGURE_VIEWPORT")]
    ConfigureViewport,
    /// Fixed-width viewport.
    #[serde(rename = "FixedWidthViewport")]
    FixedWidthViewport,
    /// Content wider than viewport.
    #[serde(rename = "SizeContentToViewport")]
    SizeContentToViewport,
    /// Text too small to read.
    #[serde(rename = "UseLegibleFontSizes")]
    UseLegibleFontSizes,
    /// Tap targets too close together.
    #[serde(rename = "TapTargetsTooClose")]
    TapTargetsTooClose,
}

/// Rich results inspection result.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichResultsInspectionResult {
    /// Detected rich result items.
    #[serde(rename = "detectedItems")]
    pub detected_items: Option<Vec<DetectedItems>>,
    /// Overall rich results verdict.
    pub verdict: Option<Verdict>,
}

/// Detected rich result items.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedItems {
    /// List of items.
    pub items: Option<Vec<DetectedItemsItem>>,
    /// Type of rich result.
    #[serde(rename = "richResultType")]
    pub rich_result_type: Option<String>,
}

/// Individual detected rich result item.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedItemsItem {
    /// Issues with this item.
    pub issues: Option<Vec<RichResultsIssue>>,
    /// Name of the item.
    pub name: Option<String>,
}

/// Rich results issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichResultsIssue {
    /// Issue message.
    #[serde(rename = "issueMessage")]
    pub issue_message: Option<String>,
    /// Issue severity.
    pub severity: Option<Severity>,
}
