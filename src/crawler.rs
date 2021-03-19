use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use crate::handlers::Domain;
use crate::{DBPool, db};
use crate::data::CrawlResult;

pub async fn crawl(domain: Domain, pool: PgPool) {
    let res = reqwest::get(&domain.name).await.unwrap().text().await.unwrap();

    let found_urls = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(str::to_string)
        .collect::<HashSet<String>>();

    for i in found_urls {
        let res = CrawlResult {
            domain_name: domain.name.to_string(),
            url: i,
        };
        db::record_result(&pool, res).await.unwrap();
    }
}