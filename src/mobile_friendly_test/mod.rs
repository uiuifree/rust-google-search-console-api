use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestMobileFriendlyTest {
    pub url: String,
    // #[serde(rename = "requestScreenshot")]
    pub request_screenshot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMobileFriendlyTest {
    #[serde(rename = "testStatus")]
    pub test_status:TestStatus,
    #[serde(rename = "mobileFriendliness")]
    pub mobile_friendliness:MobileFriendlyTestResult,
    #[serde(rename = "mobileFriendlyIssues")]
    pub mobile_friendly_issues:Option<Vec<MobileFriendlyIssue>>,
    #[serde(rename = "resourceIssues")]
    pub resource_issues:Option<Vec<ResourceIssue>>,
    pub screenshot:Option<Image>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestStatus {
    pub status: TestStatusEnum,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TestStatusEnum {
    #[serde(rename = "TEST_STATUS_UNSPECIFIED")]
    TestStatusUnspecified,
    #[serde(rename = "COMPLETE")]
    COMPLETE,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "PAGE_UNREACHABLE")]
    PageUnreachable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileFriendlyTestResult {
    #[serde(rename = "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED")]
    MobileFriendlyTestResultUnspecified,
    #[serde(rename = "MOBILE_FRIENDLY")]
    MobileFriendly,
    #[serde(rename = "NOT_MOBILE_FRIENDLY")]
    NotMobileFriendly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileFriendlyIssue {
    pub rule: MobileFriendlyRule,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileFriendlyRule {
    #[serde(rename = "MOBILE_FRIENDLY_RULE_UNSPECIFIED")]
    MobileFriendlyRuleUnspecified,
    #[serde(rename = "USES_INCOMPATIBLE_PLUGINS")]
    UsesIncompatiblePlugins,
    #[serde(rename = "ConfigureViewport")]
    ConfigureViewport,
    #[serde(rename = "FIXED_WIDTH_VIEWPORT")]
    FixedWidthViewport,
    #[serde(rename = "SIZE_CONTENT_TO_VIEWPORT")]
    SizeContentToViewport,
    #[serde(rename = "USE_LEGIBLE_FONT_SIZES")]
    UseLegibleFontSizes,
    #[serde(rename = "TAP_TARGETS_TOO_CLOSE")]
    TapTargetsTooClose,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceIssue {
    #[serde(rename = "blockedResource")]
    pub blocked_resource: BlockedResource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockedResource {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}