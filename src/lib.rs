mod error;
mod http;
pub mod types;

pub mod search_analytics;
pub mod sitemaps;
pub mod sites;
pub mod url_inspection;
pub mod mobile_friendly_test;

use serde_json::{json, Value};
use urlencoding::encode;
use crate::{GoogleApiError};
pub use error::*;
use crate::http::HttpClient;
use crate::search_analytics::query::{SearchAnalyticsQueryRequest, SearchAnalyticsQueryResponse};
use crate::sitemaps::ResponseSitemapApiList;


pub struct SearchConsoleApi {}

impl SearchConsoleApi {
    pub fn search_analytics() -> SearchAnalyticsApi {
        SearchAnalyticsApi::default()
    }
    pub fn sitemaps() -> SitemapsApi {
        SitemapsApi::default()
    }
    pub fn sites() -> SitesApi {
        SitesApi::default()
    }
    pub fn url_inspection() -> UrlInspectionApi {
        UrlInspectionApi::default()
    }
    pub fn mobile_friendly_test() -> MobileFriendlyTestApi {
        MobileFriendlyTestApi::default()
    }
}


/// https://developers.google.com/webmaster-tools/v1/searchanalytics
#[derive(Default)]
pub struct SearchAnalyticsApi {}

impl SearchAnalyticsApi {
    pub async fn query(&self, token: &str, url: &str, requests: SearchAnalyticsQueryRequest) -> Result<SearchAnalyticsQueryResponse, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!(r#" https://www.googleapis.com/webmasters/v3/sites/{}/searchAnalytics/query"#, encode(url), ).as_str(),
            requests,
        ).await?)
    }
}

use serde::{Deserialize, Serialize};
use crate::mobile_friendly_test::{RequestMobileFriendlyTest, ResponseMobileFriendlyTest};
use crate::sites::ResponseSiteApi;
use crate::url_inspection::{RequestUrlInspection, ResponseInspectionResult};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DummyRequest {}

/// https://developers.google.com/webmaster-tools/v1/sitemaps
#[derive(Default)]
pub struct SitemapsApi {}

impl SitemapsApi {
    pub async fn delete(&self, token: &str, site_url: &str, feed_path: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::delete(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed_path)).as_str(),
            json!({}),
        ).await?)
    }
    pub async fn get(&self, token: &str, site_url: &str, feed: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::get(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed)).as_str(),
            json!({}),
        ).await?)
    }
    pub async fn list(&self, token: &str, site_url: &str) -> Result<ResponseSitemapApiList, GoogleApiError> {
        Ok(HttpClient::get(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps", encode(site_url)).as_str(),
            DummyRequest::default(),
        ).await?)
    }
    pub async fn put(&self, token: &str, site_url: &str, feed: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed)).as_str(),
            json!({}),
        ).await?)
    }
    pub async fn submit(&self, token: &str, site_url: &str, feed: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::put(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed)).as_str(),
            json!({}),
        ).await?)
    }
}


/// https://developers.google.com/webmaster-tools/v1/sites?
#[derive(Default)]
pub struct SitesApi {}

impl SitesApi {
    pub async fn add(&self, token: &str, site_url: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::put(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}", encode(site_url)).as_str(),
            DummyRequest::default(),
        ).await?)
    }
    pub async fn delete(&self, token: &str, site_url: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::delete(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}", encode(site_url)).as_str(),
            DummyRequest::default(),
        ).await?)
    }
    pub async fn get(&self, token: &str, site_url: &str) -> Result<ResponseSiteApi, GoogleApiError> {
        Ok(HttpClient::get(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}", encode(site_url)).as_str(),
            DummyRequest::default(),
        ).await?)
    }
    pub async fn list(&self, token: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::get(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites").as_str(),
            DummyRequest::default(),
        ).await?)
    }
}

/// https://developers.google.com/webmaster-tools/v1/sites?
#[derive(Default)]
pub struct UrlInspectionApi {}

impl UrlInspectionApi {
    pub async fn inspect(&self, token: &str, request: &RequestUrlInspection) -> Result<ResponseInspectionResult, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!("https://searchconsole.googleapis.com/v1/urlInspection/index:inspect").as_str(),
            request,
        ).await?)
    }
}

/// https://developers.google.com/webmaster-tools/v1/sites?
#[derive(Default)]
pub struct MobileFriendlyTestApi {}

impl MobileFriendlyTestApi {
    pub async fn run(&self, api_key: &str, request: &RequestMobileFriendlyTest) -> Result<ResponseMobileFriendlyTest, GoogleApiError> {
        Ok(HttpClient::post(
            "",
            format!("https://searchconsole.googleapis.com/v1/urlTestingTools/mobileFriendlyTest:run?key={}",api_key).as_str(),
            request,
        ).await?)
    }
}
