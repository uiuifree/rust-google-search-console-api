//! Mobile Friendly Test API types.
//!
//! Types for testing whether pages are mobile-friendly.

use serde_derive::{Deserialize, Serialize};

/// Request parameters for the mobile-friendly test.
///
/// # Example
///
/// ```rust
/// use google_search_console_api::mobile_friendly_test::RequestMobileFriendlyTest;
///
/// let request = RequestMobileFriendlyTest::new("https://example.com/")
///     .with_screenshot();
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestMobileFriendlyTest {
    /// The URL to test.
    pub url: String,
    /// Whether to request a screenshot.
    #[serde(rename = "requestScreenshot")]
    pub request_screenshot: bool,
}

impl RequestMobileFriendlyTest {
    /// Create a new request with the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to test for mobile-friendliness
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            request_screenshot: false,
        }
    }

    /// Request a screenshot of the rendered page.
    ///
    /// The screenshot will be included in the response as base64-encoded data.
    pub fn with_screenshot(mut self) -> Self {
        self.request_screenshot = true;
        self
    }
}

/// Response from the mobile-friendly test.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMobileFriendlyTest {
    /// Status of the test.
    #[serde(rename = "testStatus")]
    pub test_status: TestStatus,
    /// Whether the page is mobile-friendly.
    #[serde(rename = "mobileFriendliness")]
    pub mobile_friendliness: MobileFriendlyTestResult,
    /// List of mobile-friendly issues found.
    #[serde(rename = "mobileFriendlyIssues")]
    pub mobile_friendly_issues: Option<Vec<MobileFriendlyIssue>>,
    /// List of resource loading issues.
    #[serde(rename = "resourceIssues")]
    pub resource_issues: Option<Vec<ResourceIssue>>,
    /// Screenshot of the page (if requested).
    pub screenshot: Option<Image>,
}

/// Status of the mobile-friendly test.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestStatus {
    /// Status code.
    pub status: TestStatusEnum,
    /// Additional details about the status.
    pub details: Option<String>,
}

/// Test status codes.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TestStatusEnum {
    /// Unspecified status.
    #[serde(rename = "TEST_STATUS_UNSPECIFIED")]
    TestStatusUnspecified,
    /// Test completed successfully.
    #[serde(rename = "COMPLETE")]
    COMPLETE,
    /// Internal error occurred.
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    /// Page could not be reached.
    #[serde(rename = "PAGE_UNREACHABLE")]
    PageUnreachable,
}

/// Mobile-friendly test result.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileFriendlyTestResult {
    /// Unspecified result.
    #[serde(rename = "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED")]
    MobileFriendlyTestResultUnspecified,
    /// Page is mobile-friendly.
    #[serde(rename = "MOBILE_FRIENDLY")]
    MobileFriendly,
    /// Page is not mobile-friendly.
    #[serde(rename = "NOT_MOBILE_FRIENDLY")]
    NotMobileFriendly,
}

/// A mobile-friendly issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobileFriendlyIssue {
    /// The rule that was violated.
    pub rule: MobileFriendlyRule,
}

/// Mobile-friendly rules.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MobileFriendlyRule {
    /// Unspecified rule.
    #[serde(rename = "MOBILE_FRIENDLY_RULE_UNSPECIFIED")]
    MobileFriendlyRuleUnspecified,
    /// Uses incompatible plugins (e.g., Flash).
    #[serde(rename = "USES_INCOMPATIBLE_PLUGINS")]
    UsesIncompatiblePlugins,
    /// Viewport not configured.
    #[serde(rename = "ConfigureViewport")]
    ConfigureViewport,
    /// Fixed-width viewport.
    #[serde(rename = "FIXED_WIDTH_VIEWPORT")]
    FixedWidthViewport,
    /// Content wider than screen.
    #[serde(rename = "SIZE_CONTENT_TO_VIEWPORT")]
    SizeContentToViewport,
    /// Text too small to read.
    #[serde(rename = "USE_LEGIBLE_FONT_SIZES")]
    UseLegibleFontSizes,
    /// Tap targets too close together.
    #[serde(rename = "TAP_TARGETS_TOO_CLOSE")]
    TapTargetsTooClose,
}

/// A resource loading issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceIssue {
    /// Information about the blocked resource.
    #[serde(rename = "blockedResource")]
    pub blocked_resource: BlockedResource,
}

/// Information about a blocked resource.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockedResource {
    /// URL of the blocked resource.
    pub url: String,
}

/// Screenshot image data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    /// Base64-encoded image data.
    pub data: String,
    /// MIME type of the image (e.g., "image/png").
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_new() {
        let request = RequestMobileFriendlyTest::new("https://example.com/");
        assert_eq!(request.url, "https://example.com/");
        assert!(!request.request_screenshot);
    }

    #[test]
    fn test_request_with_screenshot() {
        let request = RequestMobileFriendlyTest::new("https://example.com/").with_screenshot();
        assert_eq!(request.url, "https://example.com/");
        assert!(request.request_screenshot);
    }

    #[test]
    fn test_request_default() {
        let request = RequestMobileFriendlyTest::default();
        assert_eq!(request.url, "");
        assert!(!request.request_screenshot);
    }

    #[test]
    fn test_request_serialize() {
        let request = RequestMobileFriendlyTest::new("https://example.com/").with_screenshot();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"url\":\"https://example.com/\""));
        assert!(json.contains("\"requestScreenshot\":true"));
    }

    #[test]
    fn test_response_deserialize() {
        let json = r#"{
            "testStatus": {
                "status": "COMPLETE"
            },
            "mobileFriendliness": "MOBILE_FRIENDLY"
        }"#;

        let response: ResponseMobileFriendlyTest = serde_json::from_str(json).unwrap();
        assert!(matches!(response.test_status.status, TestStatusEnum::COMPLETE));
        assert!(matches!(
            response.mobile_friendliness,
            MobileFriendlyTestResult::MobileFriendly
        ));
    }

    #[test]
    fn test_response_with_issues() {
        let json = r#"{
            "testStatus": {
                "status": "COMPLETE"
            },
            "mobileFriendliness": "NOT_MOBILE_FRIENDLY",
            "mobileFriendlyIssues": [
                {"rule": "TAP_TARGETS_TOO_CLOSE"}
            ]
        }"#;

        let response: ResponseMobileFriendlyTest = serde_json::from_str(json).unwrap();
        assert!(matches!(
            response.mobile_friendliness,
            MobileFriendlyTestResult::NotMobileFriendly
        ));

        let issues = response.mobile_friendly_issues.unwrap();
        assert_eq!(issues.len(), 1);
        assert!(matches!(
            issues[0].rule,
            MobileFriendlyRule::TapTargetsTooClose
        ));
    }

    #[test]
    fn test_test_status_enum_deserialize() {
        assert!(matches!(
            serde_json::from_str::<TestStatusEnum>("\"COMPLETE\"").unwrap(),
            TestStatusEnum::COMPLETE
        ));
        assert!(matches!(
            serde_json::from_str::<TestStatusEnum>("\"INTERNAL_ERROR\"").unwrap(),
            TestStatusEnum::InternalError
        ));
        assert!(matches!(
            serde_json::from_str::<TestStatusEnum>("\"PAGE_UNREACHABLE\"").unwrap(),
            TestStatusEnum::PageUnreachable
        ));
    }
}
