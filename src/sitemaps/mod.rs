//! Sitemaps API types.

use crate::types::Sitemap;
use serde::{Deserialize, Serialize};

/// Response from the Sitemaps list API.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSitemapApiList {
    /// List of sitemaps for the site.
    pub sitemap: Vec<Sitemap>,
}
