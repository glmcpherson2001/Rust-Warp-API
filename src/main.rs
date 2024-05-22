use std::sync::{Arc, Mutex};

mod handlers;
mod models;
mod routes;
mod db;

use crate::db::PostDb;
use crate::models::Post;

#[tokio::main]
async fn main() {
    // Define the address and port
    let addr = ([127, 0, 0, 1], 3000);
    let db = Arc::new(Mutex::new(PostDb::new()));

    {
        let mut db = db.lock().unwrap();
        db.add_post(Post { id: 1, title: "First Post".to_string(), body: "My First Post".to_string()});
        db.add_post(Post { id: 2, title: "Second Post".to_string(), body: "My Second Post".to_string()});
    }

    // Create the routes
    let routes = routes::routes(db);

    // Start the server
    println!(
        "Server started at http://{}:{}",
        array_to_ip(addr.0),
        addr.1
    );
    warp::serve(routes).run(addr).await;
}

fn array_to_ip(ip: [u8; 4]) -> String {
    ip.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(".")
}
