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
    cfg.host = Some("127.0.0.1".to_string());
    cfg.port = Some(7878);
    cfg.user = Some("postgres".to_string());
    cfg.dbname = Some("postgres".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(NoTls).unwrap();

    db::init_db(&pool)
        .await
        .expect("database can be initialized");

    let routes = crawl_route(pool.clone())
        .or(count_route(pool.clone()))
        .or(list_route(pool.clone()))
        .recover(error::handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 4030))
        .await;
}




