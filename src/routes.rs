use warp::Filter;
use std::convert::Infallible;
use crate::handlers::{crawl_handler, count_handler, list_handler};
use deadpool_postgres::Pool;

pub fn crawl_route(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("crawl")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(crawl_handler)
}

pub fn count_route(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("count")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(count_handler)
}

pub fn list_route(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("list")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(list_handler)
}

fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}