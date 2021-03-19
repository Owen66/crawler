use warp::{Reply, Rejection, reject};
use warp::http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use crate::crawler::crawl;
use deadpool_postgres::Pool;
use crate::db;

#[derive(Deserialize, Serialize)]
pub struct Domain {
    pub name: String
}

pub async fn crawl_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    tokio::spawn(async move {
        crawl(domain, pool).await
    });

    Ok(StatusCode::ACCEPTED)
}

pub async fn count_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    let results =  db::fetch_results(&pool, Option::from(domain.name))
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(results.len().to_string())
}

pub async fn list_handler(domain: Domain, pool: Pool) -> Result<impl Reply, Rejection> {
    let results =  db::fetch_results(&pool, Option::from(domain.name))
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(warp::reply::json(
        &results
    ))
}