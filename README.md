# google-search-console-api

[![Crates.io](https://img.shields.io/crates/v/google-search-console-api.svg)](https://crates.io/crates/google-search-console-api)
[![Documentation](https://docs.rs/google-search-console-api/badge.svg)](https://docs.rs/google-search-console-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust client library for the [Google Search Console API](https://developers.google.com/webmaster-tools/v1/api_reference_index).

## Features

- **Search Analytics** - Query search performance data (clicks, impressions, CTR, position)
- **Sitemaps** - List, get, submit, and delete sitemaps
- **Sites** - Add, delete, get, and list sites
- **URL Inspection** - Inspect URLs for indexing status, AMP, mobile usability, and rich results
- **Mobile Friendly Test** - Test if a page is mobile-friendly

## Installation

```toml
[dependencies]
google-search-console-api = "0.1"
```

Or using cargo:

```bash
cargo add google-search-console-api
```

## Quick Start

### Authentication

This library requires an OAuth2 access token. We recommend using [yup-oauth2](https://crates.io/crates/yup-oauth2) for authentication.

```rust
use yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key};

async fn get_token() -> String {
    let secret = read_service_account_key("service-account.json")
        .await
        .expect("Failed to read service account key");

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .expect("Failed to create authenticator");

    let scopes = &["https://www.googleapis.com/auth/webmasters"];
    let token = auth.token(scopes).await.expect("Failed to get token");

    token.token().unwrap().to_string()
}
```

### Search Analytics

Query search performance data for your site:

```rust
use google_search_console_api::SearchConsoleApi;
use google_search_console_api::search_analytics::query::SearchAnalyticsQueryRequest;
use google_search_console_api::types::Dimension;

#[tokio::main]
async fn main() {
    let token = get_token().await;
    let site_url = "https://example.com/";

    // Using the builder pattern (recommended)
    let request = SearchAnalyticsQueryRequest::builder("2024-01-01", "2024-01-31")
        .dimensions(vec![Dimension::Query, Dimension::Page])
        .row_limit(100)
        .build();

    let response = SearchConsoleApi::search_analytics()
        .query(&token, site_url, request)
        .await
        .expect("Failed to query search analytics");

    if let Some(rows) = response.rows {
        for row in rows {
            println!(
                "Keys: {:?}, Clicks: {:?}, Impressions: {:?}, CTR: {:?}, Position: {:?}",
                row.keys, row.clicks, row.impressions, row.ctr, row.position
            );
        }
    }
}
```

### Sitemaps

Manage sitemaps for your site:

```rust
use google_search_console_api::SearchConsoleApi;

#[tokio::main]
async fn main() {
    let token = get_token().await;
    let site_url = "https://example.com/";

    // List all sitemaps
    let sitemaps = SearchConsoleApi::sitemaps()
        .list(&token, site_url)
        .await
        .expect("Failed to list sitemaps");

    println!("Sitemaps: {:?}", sitemaps);

    // Submit a new sitemap
    SearchConsoleApi::sitemaps()
        .submit(&token, site_url, "https://example.com/sitemap.xml")
        .await
        .expect("Failed to submit sitemap");
}
```

### Sites

Manage sites in your Search Console:

```rust
use google_search_console_api::SearchConsoleApi;

#[tokio::main]
async fn main() {
    let token = get_token().await;

    // List all sites
    let sites = SearchConsoleApi::sites()
        .list(&token)
        .await
        .expect("Failed to list sites");

    println!("Sites: {:?}", sites);

    // Get a specific site
    let site = SearchConsoleApi::sites()
        .get(&token, "https://example.com/")
        .await
        .expect("Failed to get site");

    println!("Site: {:?}", site);
}
```

### URL Inspection

Inspect a URL for indexing status:

```rust
use google_search_console_api::SearchConsoleApi;
use google_search_console_api::url_inspection::RequestUrlInspection;

#[tokio::main]
async fn main() {
    let token = get_token().await;

    let request = RequestUrlInspection {
        inspection_url: "https://example.com/page".to_string(),
        site_url: "https://example.com/".to_string(),
        language_code: "en".to_string(),
    };

    let result = SearchConsoleApi::url_inspection()
        .inspect(&token, &request)
        .await
        .expect("Failed to inspect URL");

    println!("Inspection result: {:?}", result);
}
```

### Mobile Friendly Test

Test if a page is mobile-friendly (requires API key):

```rust
use google_search_console_api::SearchConsoleApi;
use google_search_console_api::mobile_friendly_test::RequestMobileFriendlyTest;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY";

    // Simple request
    let request = RequestMobileFriendlyTest::new("https://example.com/");

    // Or with screenshot
    let request_with_screenshot = RequestMobileFriendlyTest::new("https://example.com/")
        .with_screenshot();

    let result = SearchConsoleApi::mobile_friendly_test()
        .run(api_key, &request)
        .await
        .expect("Failed to run mobile friendly test");

    println!("Mobile friendly: {:?}", result);
}
```

## API Reference

- [Search Analytics](https://developers.google.com/webmaster-tools/v1/searchanalytics)
- [Sitemaps](https://developers.google.com/webmaster-tools/v1/sitemaps)
- [Sites](https://developers.google.com/webmaster-tools/v1/sites)
- [URL Inspection](https://developers.google.com/webmaster-tools/v1/urlInspection.index)

## License

MIT License - see [LICENSE](LICENSE) for details.
