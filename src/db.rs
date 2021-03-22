use deadpool_postgres::Pool;
use crate::data::{CrawlResult, CountResponse, ListResponse};
use crate::error::Error::{DBQueryError};
use crate::error;

type Result<T> = std::result::Result<T, error::Error>;

const TABLE: &str = "results";
const SELECT_FIELDS: &str = "id, domain_name, url";

pub async fn record_result(pool: &Pool, result: CrawlResult) -> Result<()> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (domain_name, url) VALUES ($1,$2) RETURNING *", TABLE);
    let stmt = client.prepare(query.as_str()).await?;
    client.query(&stmt, &[&result.domain_name, &result.url]).await?;
    Ok(())
}

pub async fn fetch_results(pool: &Pool, search: String) -> Result<ListResponse> {
    let client = pool.get().await.unwrap();
    let query = format!(
        "SELECT {} FROM {} {}",
        SELECT_FIELDS, TABLE, "WHERE domain_name like $1"
    );
    let stmt = client.prepare(query.as_str()).await?;
    let q = client.query(&stmt, &[&search]).await;
    let rows = q.map_err(DBQueryError)?;
    let results: Vec<String> = rows.iter().map(|r|r.get(2)).collect();
    Ok(ListResponse {
        links: results,
    })
}

pub async fn count_results(pool: &Pool, search: String) -> Result<CountResponse> {
    let client = pool.get().await.unwrap();
    let query = format!(
        "SELECT COUNT(url) AS count FROM {} {} {}",
        TABLE, "WHERE domain_name like $1", "GROUP BY domain_name"
    );
    let stmt = client.prepare(query.as_str()).await?;
    let q = client.query(&stmt, &[&search]).await;
    let rows = q.map_err(DBQueryError)?;
    let row = rows.get(0).unwrap();
    let count = CountResponse {
        count: row.get(0),
    };
    Ok(count)
}
