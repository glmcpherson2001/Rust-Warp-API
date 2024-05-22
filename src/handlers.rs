use crate::models::Post;
use warp;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Post {
        id,
        title: "Static Post".to_string(),
        body: "Static Body Section".to_string(),
    };

    Ok(warp::reply::json(&post))
}
