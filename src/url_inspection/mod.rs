use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestUrlInspection {
    #[serde(rename = "inspectionUrl")]
    pub inspection_url: String,
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    #[serde(rename = "languageCode")]
    pub language_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseInspectionResult {
    #[serde(rename = "inspectionResult")]
    pub inspection_result: InspectionResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InspectionResult {
    #[serde(rename = "inspectionResultLink")]
    pub inspection_result_link: Option<String>,
    #[serde(rename = "indexStatusResult")]
    pub index_status_result: Option<IndexStatusResult>,
    #[serde(rename = "ampResult")]
    pub amp_result: Option<AmpInspectionResult>,
    #[serde(rename = "mobileUsabilityResult")]
    pub mobile_usability_result: Option<MobileUsabilityInspectionResult>,
    #[serde(rename = "richResultsResult")]
    pub rich_results_result: Option<RichResultsInspectionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexStatusResult {
    pub sitemap: Option<Vec<String>>,
    #[serde(rename = "referringUrls")]
    pub referring_urls: Option<Vec<String>>,
    pub verdict: Option<Verdict>,
    #[serde(rename = "coverageState")]
    pub coverage_state: Option<String>,
    #[serde(rename = "robotsTxtState")]
    pub robots_txt_state: Option<RobotsTxtState>,
    #[serde(rename = "indexingState")]
    pub indexing_state: Option<IndexingState>,
    #[serde(rename = "lastCrawlTime")]
    pub last_crawl_time: Option<String>,
    #[serde(rename = "pageFetchState")]
    pub page_fetch_state: Option<PageFetchState>,
    #[serde(rename = "googleCanonical")]
    pub google_canonical: Option<String>,
    #[serde(rename = "userCanonical")]
    pub user_canonical: Option<String>,
    #[serde(rename = "crawledAs")]
    pub crawled_as: Option<CrawlUserAgent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Verdict {
    #[serde(rename = "VERDICT_UNSPECIFIED")]
    VerdictUnspecified,
    PASS,
    PARTIAL,
    FAIL,
    NEUTRAL,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RobotsTxtState {
    #[serde(rename = "ROBOTS_TXT_STATE_UNSPECIFIED")]
    RobotsTxtStateUnspecified,
    ALLOWED,
    DISALLOWED,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IndexingState {
    #[serde(rename = "INDEXING_STATE_UNSPECIFIED")]
    IndexingStateUnspecified,
    #[serde(rename = "INDEXING_ALLOWED")]
    IndexingAllowed,
    #[serde(rename = "BLOCKED_BY_META_TAG")]
    BlockedByMetaTag,
    #[serde(rename = "BLOCKED_BY_HTTP_HEADER")]
    BlockedByHttpHeader,
    #[serde(rename = "BLOCKED_BY_ROBOTS_TXT")]
    BlockedByRobotsTxt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PageFetchState {
    #[serde(rename = "PAGE_FETCH_STATE_UNSPECIFIED")]
    PageFetchStateUnspecified,
    SUCCESSFUL,
    #[serde(rename = "SOFT_404")]
    Soft404,
    #[serde(rename = "BLOCKED_ROBOTS_TXT")]
    BlockedRobotsTxt,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "ACCESS_DENIED")]
    AccessDenied,
    #[serde(rename = "SERVER_ERROR")]
    ServerError,
    #[serde(rename = "REDIRECT_ERROR")]
    RedirectError,
    #[serde(rename = "ACCESS_FORBIDDEN")]
    AccessForbidden,
    #[serde(rename = "BLOCKED_4XX")]
    Blocked4xx,
    #[serde(rename = "INTERNAL_CRAWL_ERROR")]
    InternalCrawlError,
    #[serde(rename = "INVALID_URL")]
    InvalidUrl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrawlUserAgent {
    #[serde(rename = "CRAWLING_USER_AGENT_UNSPECIFIED")]
    CrawlingUserAgentUnspecified,
    DESKTOP,
    MOBILE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmpInspectionResult {
    issues: Option<Vec<AmpIssue>>,
    verdict: Option<Verdict>,
    #[serde(rename = "ampUrl")]
    amp_url: Option<String>,
    #[serde(rename = "robotsTxtState")]
    robots_txt_state: Option<RobotsTxtState>,
    #[serde(rename = "indexingState")]
    indexing_state: Option<AmpIndexingState>,
    #[serde(rename = "ampIndexStatusVerdict")]
    amp_index_status_verdict: Option<Verdict>,
    #[serde(rename = "lastCrawlTime")]
    last_crawl_time: Option<String>,
    #[serde(rename = "pageFetchState")]
    page_fetch_state: Option<PageFetchState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AmpIndexingState {
    #[serde(rename = "AMP_INDEXING_STATE_UNSPECIFIED")]
    AmpIndexingStateUnspecified,
    #[serde(rename = "AMP_INDEXING_ALLOWED")]
    AmpIndexingAllowed,
    #[serde(rename = "BLOCKED_DUE_TO_NOINDEX")]
    BlockedDueToNoindex,
    #[serde(rename = "BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER")]
    BlockedDueToExpiredUnavailableAfter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmpIssue {
    #[serde(rename = "issueMessage")]
    pub issue_message: Option<String>,
    pub severity: Option<Severity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    SeverityUnspecified,
    WARNING,
    ERROR,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileUsabilityInspectionResult {
    pub issues: Option<Vec<MobileUsabilityIssue>>,
    pub verdict: Option<Verdict>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileUsabilityIssue {
    #[serde(rename = "issueType")]
    pub issue_type: Option<MobileUsabilityIssueType>,
    pub severity: Option<Severity>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileUsabilityIssueType {
    #[serde(rename = "MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED")]
    MobileUsabilityIssueTypeUnspecified,
    #[serde(rename = "USES_INCOMPATIBLE_PLUGINS")]
    UsesIncompatiblePlugins,
    #[serde(rename = "CONFIGURE_VIEWPORT")]
    ConfigureViewport,
    #[serde(rename = "FixedWidthViewport")]
    FixedWidthViewport,
    #[serde(rename = "SizeContentToViewport")]
    SizeContentToViewport,
    #[serde(rename = "UseLegibleFontSizes")]
    UseLegibleFontSizes,
    #[serde(rename = "TapTargetsTooClose")]
    TapTargetsTooClose,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichResultsInspectionResult {
    #[serde(rename = "detectedItems")]
    pub detected_items: Option<Vec<DetectedItems>>,
    pub verdict: Option<Verdict>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedItems {
    pub items: Option<Vec<DetectedItemsItem>>,
    #[serde(rename = "richResultType")]
    pub rich_result_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedItemsItem {
    pub issues: Option<Vec<RichResultsIssue>>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichResultsIssue {
    #[serde(rename = "issueMessage")]
    pub issue_message: Option<String>,
    pub severity: Option<Severity>,
}