use std::sync::{Arc, Mutex};

// use crate::models::Post;
use crate::db::PostDb;
use crate::models::{CreatePost, UpdatePost};
use warp::{http::StatusCode, Rejection, Reply};
use warp::reply::json;

pub async fn get_all_posts(db: Arc<Mutex<PostDb>>) -> Result<impl Reply, Rejection> {
    let db = db.lock().unwrap();

    let posts = db.get_all_posts();

    Ok(json(&posts))
}

pub async fn get_post(id: u64, db: Arc<Mutex<PostDb>>) -> Result<impl Reply, Rejection> {
    // Lock the database to access it
    let mut db = db.lock().unwrap();

    // Retrieve the post by id
    if let Some(post) = db.get_post(id) {
        Ok(json(post))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn create_post(post: CreatePost, db: Arc<Mutex<PostDb>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();

    let new_post = db.add_post(post);

    Ok(json(&new_post))
}

pub async fn update_post(id: u64, updated_post: UpdatePost, db: Arc<Mutex<PostDb>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    
    if let Some(post) = db.update_post(id, updated_post.title, updated_post.body){
        Ok(json(post))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn delete_post(id: u64, db:Arc<Mutex<PostDb>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();

    if db.delete_post(id) {
        Ok(warp::reply::with_status("Post Deleted", StatusCode::NO_CONTENT))
    } else {
        Ok(warp::reply::with_status("Post Not found", StatusCode::NOT_FOUND))
    }
} 