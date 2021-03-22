use warp::{Reply, Rejection, reject};
use warp::http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use crate::crawler::crawl;
use deadpool_postgres::Pool;
use url::Url;
use crate::db;
use crate::error::Error::{UrlParseError};
use crate::error::Error;

#[derive(Deserialize, Serialize)]
pub struct Domain {
    pub name: String
}

pub async fn crawl_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    let valid_domain = clean_input(domain).map_err(|e| reject::custom(e))?;
    tokio::spawn(async move {
        crawl(valid_domain, pool).await
    });

    Ok(StatusCode::ACCEPTED)
}

pub async fn count_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    let valid_domain = clean_input(domain).map_err(|e| reject::custom(e))?;
    let count =  db::count_results(&pool, valid_domain)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(
        &count
    ))
}

pub async fn list_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    let valid_domain = clean_input(domain).map_err(|e| reject::custom(e))?;
    let results =  db::fetch_results(&pool, valid_domain)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(warp::reply::json(
        &results
    ))
}

fn clean_input(domain: Domain) -> Result<String, Error>{
    let mut dname = Url::parse(domain.name.as_str()).map_err(UrlParseError)?;
    dname.set_path("");
    dname.set_query(None);
    return Ok(dname.to_string());
}

#[test]
fn test_clean_input_passes_good_input() {
    let d = Domain{
        name: "http://www.theregister.com/crawl/hmm/testing?orang=4".to_string()
    };
    assert_eq!(clean_input(d).unwrap(), "http://www.theregister.com/");
}

#[test]
fn test_clean_input_rejects_bad_input() {
    let d = Domain{
        name: "theregister.com".to_string()
    };

    let result = match clean_input(d) {
        Ok(_) => false,
        Err(Error::UrlParseError(_)) => true,
        Err(_) => false,
    };
    assert!(result);
}
