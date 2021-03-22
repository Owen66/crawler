use select::document::Document;
use select::predicate::Name;
use std::collections::{HashSet, VecDeque};
use crate::db;
use crate::data::CrawlResult;
use deadpool_postgres::Pool;
use url::{ParseError, Url};

pub async fn crawl(domain: String, pool: Pool) {
    let mut domain_url = Url::parse(&domain).unwrap();
    domain_url.set_path("");
    domain_url.set_query(None);

    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(domain_url.clone());

    let results =  db::fetch_results(&pool, domain_url.to_string()).await.unwrap();
    for i in results.links {
        visited.insert(Url::parse(&i).unwrap());
    }

    while !frontier.is_empty() {
        let url = frontier.pop_front().unwrap();
        let mut url_domain = url.clone();
        url_domain.set_path("");
        url_domain.set_query(None);
        if visited.contains(&url) {
            continue;
        }
        visited.insert(url.clone());

        let page_result = reqwest::get(url.clone()).await;
        if page_result.is_err() {
            continue;
        }

        let text_result = page_result.unwrap().text().await;

        if text_result.is_err() {
            continue;
        }

        let page = text_result.unwrap();

        let found_urls = Document::from(page.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .filter_map(|link| match Url::parse(link) {
                Err(ParseError::RelativeUrlWithoutBase) => Some(url_domain.join(link).unwrap()),
                Err(_) => None,
                Ok(url) => Some(url),
            })
            .collect::<HashSet<Url>>();
        for i in found_urls {
            if !visited.contains(&i) && i.to_string().contains(&domain_from_url(domain_url.clone())) {
                frontier.push_back(i)
            }
        }
        let d = CrawlResult {
            domain_name: domain.clone(),
            url: url.to_string(),
        };

        if db::record_result(&pool, d).await.is_err() {
            continue;
        };
    }
}

fn domain_from_url(url: Url) -> String {
    let d = url.domain().unwrap();
    let r = d.replace("www.", "");
    return r;
}

#[test]
fn test_domain_from_url() {
    let u = Url::parse("https://www.wikipedia.org/").unwrap();
    assert_eq!(domain_from_url(u), "wikipedia.org");
}
