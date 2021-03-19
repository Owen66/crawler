use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Results {
    pub id: i32,
    pub domain_name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct CrawlResult {
    pub domain_name: String,
    pub url: String,
}
