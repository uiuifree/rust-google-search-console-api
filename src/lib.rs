//! # Google Search Console API Client
//!
//! An unofficial Rust client library for the Google Search Console API.
//!
//! ## Features
//!
//! - **Search Analytics** - Query search performance data
//! - **Sitemaps** - Manage sitemaps
//! - **Sites** - Manage sites
//! - **URL Inspection** - Inspect URLs for indexing status
//! - **Mobile Friendly Test** - Test mobile friendliness
//!
//! ## Example
//!
//! ```rust,no_run
//! use google_search_console_api::SearchConsoleApi;
//! use google_search_console_api::search_analytics::query::SearchAnalyticsQueryRequest;
//! use google_search_console_api::types::Dimension;
//!
//! #[tokio::main]
//! async fn main() {
//!     let token = "your_oauth_token";
//!     let site_url = "https://example.com/";
//!
//!     let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
//!         .dimensions(vec![Dimension::Query, Dimension::Page])
//!         .row_limit(100)
//!         .build();
//!
//!     let response = SearchConsoleApi::search_analytics()
//!         .query(token, site_url, request)
//!         .await;
//! }
//! ```

mod error;
mod http;

pub mod mobile_friendly_test;
pub mod search_analytics;
pub mod sitemaps;
pub mod sites;
pub mod types;
pub mod url_inspection;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use urlencoding::encode;

pub use error::*;

use crate::http::HttpClient;
use crate::mobile_friendly_test::{RequestMobileFriendlyTest, ResponseMobileFriendlyTest};
use crate::search_analytics::query::{SearchAnalyticsQueryRequest, SearchAnalyticsQueryResponse};
use crate::sitemaps::ResponseSitemapApiList;
use crate::sites::{ResponseSiteApi, ResponseSiteListApi};
use crate::url_inspection::{RequestUrlInspection, ResponseInspectionResult};

/// Main entry point for the Google Search Console API.
///
/// Provides access to all API endpoints through method chaining.
///
/// # Example
///
/// ```rust,no_run
/// use google_search_console_api::SearchConsoleApi;
///
/// # async fn example() {
/// let sites = SearchConsoleApi::sites().list("token").await;
/// # }
/// ```
pub struct SearchConsoleApi;

impl SearchConsoleApi {
    /// Access the Search Analytics API.
    ///
    /// Query search performance data including clicks, impressions, CTR, and position.
    pub fn search_analytics() -> SearchAnalyticsApi {
        SearchAnalyticsApi
    }

    /// Access the Sitemaps API.
    ///
    /// List, submit, and delete sitemaps for your sites.
    pub fn sitemaps() -> SitemapsApi {
        SitemapsApi
    }

    /// Access the Sites API.
    ///
    /// Add, remove, and list sites in your Search Console account.
    pub fn sites() -> SitesApi {
        SitesApi
    }

    /// Access the URL Inspection API.
    ///
    /// Inspect URLs for indexing status, crawl info, and more.
    pub fn url_inspection() -> UrlInspectionApi {
        UrlInspectionApi
    }

    /// Access the Mobile Friendly Test API.
    ///
    /// Test whether a page is mobile-friendly.
    pub fn mobile_friendly_test() -> MobileFriendlyTestApi {
        MobileFriendlyTestApi
    }
}

/// Search Analytics API client.
///
/// Query search traffic data for your site.
///
/// # API Reference
///
/// <https://developers.google.com/webmaster-tools/v1/searchanalytics>
#[derive(Default)]
pub struct SearchAnalyticsApi;

impl SearchAnalyticsApi {
    /// Query search analytics data.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `url` - Site URL (e.g., `https://example.com/`)
    /// * `request` - Query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use google_search_console_api::SearchConsoleApi;
    /// use google_search_console_api::search_analytics::query::SearchAnalyticsQueryRequest;
    ///
    /// # async fn example() {
    /// let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
    ///     .row_limit(100)
    ///     .build();
    ///
    /// let response = SearchConsoleApi::search_analytics()
    ///     .query("token", "https://example.com/", request)
    ///     .await;
    /// # }
    /// ```
    pub async fn query(
        &self,
        token: &str,
        url: &str,
        request: SearchAnalyticsQueryRequest,
    ) -> Result<SearchAnalyticsQueryResponse, GoogleApiError> {
        HttpClient::post(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/searchAnalytics/query",
                encode(url)
            ),
            request,
        )
        .await
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DummyRequest {}

/// Sitemaps API client.
///
/// Manage sitemaps for your sites.
///
/// # API Reference
///
/// <https://developers.google.com/webmaster-tools/v1/sitemaps>
#[derive(Default)]
pub struct SitemapsApi;

impl SitemapsApi {
    /// Delete a sitemap.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - Site URL
    /// * `feed_path` - Full URL of the sitemap to delete
    pub async fn delete(
        &self,
        token: &str,
        site_url: &str,
        feed_path: &str,
    ) -> Result<Value, GoogleApiError> {
        HttpClient::delete(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}",
                encode(site_url),
                encode(feed_path)
            ),
            json!({}),
        )
        .await
    }

    /// Get information about a specific sitemap.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - Site URL
    /// * `feed` - Full URL of the sitemap
    pub async fn get(
        &self,
        token: &str,
        site_url: &str,
        feed: &str,
    ) -> Result<Value, GoogleApiError> {
        HttpClient::get(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}",
                encode(site_url),
                encode(feed)
            ),
            json!({}),
        )
        .await
    }

    /// List all sitemaps for a site.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - Site URL
    pub async fn list(
        &self,
        token: &str,
        site_url: &str,
    ) -> Result<ResponseSitemapApiList, GoogleApiError> {
        HttpClient::get(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps",
                encode(site_url)
            ),
            DummyRequest::default(),
        )
        .await
    }

    /// Resubmit a sitemap.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - Site URL
    /// * `feed` - Full URL of the sitemap
    pub async fn put(
        &self,
        token: &str,
        site_url: &str,
        feed: &str,
    ) -> Result<Value, GoogleApiError> {
        HttpClient::post(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}",
                encode(site_url),
                encode(feed)
            ),
            json!({}),
        )
        .await
    }

    /// Submit a new sitemap.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - Site URL
    /// * `feed` - Full URL of the sitemap to submit
    pub async fn submit(
        &self,
        token: &str,
        site_url: &str,
        feed: &str,
    ) -> Result<Value, GoogleApiError> {
        HttpClient::put(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}",
                encode(site_url),
                encode(feed)
            ),
            json!({}),
        )
        .await
    }
}

/// Sites API client.
///
/// Manage sites in your Search Console account.
///
/// # API Reference
///
/// <https://developers.google.com/webmaster-tools/v1/sites>
#[derive(Default)]
pub struct SitesApi;

impl SitesApi {
    /// Add a site to Search Console.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - URL of the site to add
    pub async fn add(&self, token: &str, site_url: &str) -> Result<Value, GoogleApiError> {
        HttpClient::put(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}",
                encode(site_url)
            ),
            DummyRequest::default(),
        )
        .await
    }

    /// Remove a site from Search Console.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - URL of the site to remove
    pub async fn delete(&self, token: &str, site_url: &str) -> Result<Value, GoogleApiError> {
        HttpClient::delete(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}",
                encode(site_url)
            ),
            DummyRequest::default(),
        )
        .await
    }

    /// Get information about a specific site.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `site_url` - URL of the site
    pub async fn get(
        &self,
        token: &str,
        site_url: &str,
    ) -> Result<ResponseSiteApi, GoogleApiError> {
        HttpClient::get(
            token,
            &format!(
                "https://www.googleapis.com/webmasters/v3/sites/{}",
                encode(site_url)
            ),
            DummyRequest::default(),
        )
        .await
    }

    /// List all sites in your Search Console account.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    pub async fn list(&self, token: &str) -> Result<ResponseSiteListApi, GoogleApiError> {
        HttpClient::get(
            token,
            "https://www.googleapis.com/webmasters/v3/sites",
            DummyRequest::default(),
        )
        .await
    }
}

/// URL Inspection API client.
///
/// Inspect URLs for indexing status and other information.
///
/// # API Reference
///
/// <https://developers.google.com/webmaster-tools/v1/urlInspection.index>
#[derive(Default)]
pub struct UrlInspectionApi;

impl UrlInspectionApi {
    /// Inspect a URL.
    ///
    /// Returns indexing status, crawl info, AMP status, mobile usability, and rich results.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth2 access token
    /// * `request` - Inspection request parameters
    pub async fn inspect(
        &self,
        token: &str,
        request: &RequestUrlInspection,
    ) -> Result<ResponseInspectionResult, GoogleApiError> {
        HttpClient::post(
            token,
            "https://searchconsole.googleapis.com/v1/urlInspection/index:inspect",
            request,
        )
        .await
    }
}

/// Mobile Friendly Test API client.
///
/// Test whether a page is mobile-friendly.
///
/// # API Reference
///
/// <https://developers.google.com/webmaster-tools/v1/urlTestingTools.mobileFriendlyTest>
#[derive(Default)]
pub struct MobileFriendlyTestApi;

impl MobileFriendlyTestApi {
    /// Run a mobile-friendly test.
    ///
    /// Note: This API uses an API key instead of OAuth2.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Google API key
    /// * `request` - Test request parameters
    pub async fn run(
        &self,
        api_key: &str,
        request: &RequestMobileFriendlyTest,
    ) -> Result<ResponseMobileFriendlyTest, GoogleApiError> {
        HttpClient::post(
            "",
            &format!(
                "https://searchconsole.googleapis.com/v1/urlTestingTools/mobileFriendlyTest:run?key={}",
                api_key
            ),
            request,
        )
        .await
    }
}
