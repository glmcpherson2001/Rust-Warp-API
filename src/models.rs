use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
   pub id: u64,
   pub title: String,
   pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePost {
    pub title: String,
    pub body: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>
}