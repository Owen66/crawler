mod routes;
mod handlers;
mod crawler;
mod db;
mod data;
mod error;

use crate::routes::{crawl_route, count_route, list_route};
use warp::Filter;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use deadpool_postgres::tokio_postgres::NoTls;

#[tokio::main]
async fn main() {

    let mut cfg = Config::new();
    cfg.host = Some("crawler-db".to_string());
    cfg.port = Some(5432);
    cfg.dbname = Some("postgres".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(NoTls).unwrap();

    let routes = crawl_route(pool.clone())
        .or(count_route(pool.clone()))
        .or(list_route(pool.clone()))
        .recover(error::handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
