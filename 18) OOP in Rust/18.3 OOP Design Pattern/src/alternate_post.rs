/*
An alternate way to have states is to represent them as different types.
*/

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct  PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { 
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Take ownership of self, consuming the DraftPost, and creating a new PendingReviewPost.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { 
            content: self.content 
        }
    }
}

impl PendingReviewPost {
    // Take ownership of self, consuming the PendingReviewPost, and creating a new Post.
    pub fn approve(self) -> Post {
        Post { 
            content: self.content 
        }
    }
}