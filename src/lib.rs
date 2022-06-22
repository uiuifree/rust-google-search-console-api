mod error;
mod http;
pub mod types;

pub mod search_analytics;

use serde_json::json;
use urlencoding::encode;
use crate::{GoogleApiError};
pub use error::*;
use crate::http::HttpClient;
use crate::search_analytics::query::{SearchAnalyticsQueryRequest, SearchAnalyticsQueryResponse};


pub struct SearchConsoleApi {}
impl SearchConsoleApi {
    pub fn search_analytics()->SearchAnalyticsApi{
        SearchAnalyticsApi::default()
    }
}



/// https://developers.google.com/webmaster-tools/v1/searchanalytics
#[derive(Default)]
pub struct SearchAnalyticsApi {}


impl SearchAnalyticsApi{
    pub async fn query(&self,token: &str, url: &str, requests: SearchAnalyticsQueryRequest) -> Result<SearchAnalyticsQueryResponse, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!(r#" https://www.googleapis.com/webmasters/v3/sites/{}/searchAnalytics/query"#, encode(url), ).as_str(),
            requests,
        ).await?)
    }
}
