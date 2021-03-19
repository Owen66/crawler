use deadpool_postgres::Pool;
use deadpool_postgres::tokio_postgres::Row;
use crate::data::{CrawlResult, Results};
use crate::error::Error::{DBQueryError, DBInitError};
use crate::error;
use std::fs;

type Result<T> = std::result::Result<T, error::Error>;

const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "results";
const SELECT_FIELDS: &str = "id, domain_name, url";

pub async fn init_db(pool: &Pool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let client = pool.get().await.unwrap();
    client.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn record_result(pool: &Pool, result: CrawlResult) -> Result<()> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (domain_name, url) VALUES ($1,$2) RETURNING *", TABLE);
    let stmt = client.prepare(query.as_str()).await?;
    client.query(&stmt, &[&result.domain_name, &result.url]).await?;
    Ok(())
}

pub async fn fetch_results(pool: &Pool, search: Option<String>) -> Result<Vec<Results>> {
    let client = pool.get().await.unwrap();
    let where_clause = match search {
        Some(_) => "WHERE domain_name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {}",
        SELECT_FIELDS, TABLE, where_clause
    );
    let stmt = client.prepare(query.as_str()).await?;
    let q = match search {
        Some(v) => client.query(&stmt, &[&v]).await,
        None => client.query(&stmt, &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_result(&r)).collect())
}


fn row_to_result(row: &Row) -> Results {
    let id: i32 = row.get(0);
    let domain_name: String = row.get(1);
    let url: String = row.get(2);
    Results {
        id,
        domain_name,
        url,
    }
}