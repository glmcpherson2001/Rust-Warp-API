use crate::models::Post;

#[derive(Default)]
pub struct PostDb {
    posts: Vec<Post>
}

impl PostDb {
    pub fn new() -> Self {
        Self { posts: Vec::new() }
    }

    pub fn add_post(&mut self, post: Post){
        self.posts.push(post)
    }

    pub fn get_post(&mut self, id: u64) -> Option<&Post> {
        self.posts.iter().find(|&post| post.id == id)
    }
    
    pub fn get_all_posts(&self) -> Vec<&Post> {
        self.posts.iter().collect()
    }

    pub fn update_post(&mut self, id: u64, title: Option<String>, body: Option<String>) -> Option<&Post> {
        if let Some(post) = self.posts.iter_mut().find(|post| post.id == id) {
            if let Some(new_title) = title {
                post.title = new_title;
            }

            if let Some(new_body) = body {
                post.body = new_body;
            }

            Some(post)
        } else {
            None
        }
    }

    pub fn delete_post(&mut self, id: u64) -> bool {
        if let Some(index) = self.posts.iter().position(|post| post.id == id) {
            self.posts.remove(index);
            true
        } else {
            false
        }
    }

    
 }