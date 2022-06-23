use crate::types::Sitemap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseSitemapApiList {
    pub sitemap: Vec<Sitemap>,
}
