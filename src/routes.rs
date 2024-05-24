use crate::handlers;
use crate::db::PostDb;
use warp::Filter;
use std::sync::{Arc, Mutex};

pub fn routes(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post(db.clone())
    .or(get_all_posts(db.clone()))
    .or(create_post(db.clone()))
    .or(update_post(db.clone()))
    .or(delete_post(db))
}

fn get_post(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_post)
}

fn get_all_posts(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_all_posts)
}

fn create_post(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("posts")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_post)
}

fn update_post(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::update_post)
}

fn delete_post(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_post)
}

fn with_db(
    db: Arc<Mutex<PostDb>>
) -> impl Filter<Extract = (Arc<Mutex<PostDb>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}