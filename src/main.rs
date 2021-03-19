mod routes;
mod handlers;
mod crawler;
mod db;
mod data;
mod error;

use crate::routes::{crawl_route, count_route, list_route};
use warp::Filter;
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    let routes = crawl_route(pool.clone())
        .or(count_route(pool.clone()))
        .or(list_route(pool.clone()))
        .recover(error::handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 4030))
        .await;
}




