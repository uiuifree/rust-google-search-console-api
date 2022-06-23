mod error;
mod http;
pub mod types;

pub mod search_analytics;
pub mod sitemaps;

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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DummyRequest{

}

/// https://developers.google.com/webmaster-tools/v1/sitemaps
#[derive(Default)]
pub struct SitemapsApi {}

impl SitemapsApi {
    pub async fn delete(&self, token: &str, site_url: &str, feed_path: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::delete(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed_path)).as_str(),
            json!({})
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
    pub async fn submit(&self, token: &str,site_url: &str, feed: &str) -> Result<Value, GoogleApiError> {
        Ok(HttpClient::put(
            token,
            format!("https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}", encode(site_url), encode(feed)).as_str(),
            json!({}),
        ).await?)
    }
}
