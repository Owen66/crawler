use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "results";
const SELECT_FIELDS: &str = "id, domain_name, url";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn record_result(db_pool: &DBPool, result: CrawlResult) -> Result<()> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (domain_name, url) VALUES ($1,$2) RETURNING *", TABLE);
    con.query_one(query.as_str(), &[&result.domain_name, &result.url])
        .await
        .map_err(DBQueryError)?;
    Ok(())
}

pub async fn fetch_results(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Results>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE domain_name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {}",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
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