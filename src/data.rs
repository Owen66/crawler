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
#[derive(Deserialize, Serialize)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Deserialize, Serialize)]
pub struct ListResponse {
    pub links: Vec<String>,
}
