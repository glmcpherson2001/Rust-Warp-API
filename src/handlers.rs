use std::sync::{Arc, Mutex};

// use crate::models::Post;
use crate::db::PostDb;
use warp;
use warp::reply::json;

pub async fn get_post(id: u64, db: Arc<Mutex<PostDb>>) -> Result<impl warp::Reply, warp::Rejection> {
    // Lock the database to access it
    let mut db = db.lock().unwrap();

    // Retrieve the post by id
    if let Some(post) = db.get_post(id) {
        Ok(json(post))
    } else {
        Err(warp::reject::not_found())
    }
}
