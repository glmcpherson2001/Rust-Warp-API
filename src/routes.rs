use crate::handlers;
use crate::db::PostDb;
use warp::Filter;
use std::sync::{Arc, Mutex};

pub fn routes(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post(db)
}

fn get_post(db: Arc<Mutex<PostDb>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_post)
}

fn with_db(
    db: Arc<Mutex<PostDb>>
) -> impl Filter<Extract = (Arc<Mutex<PostDb>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}