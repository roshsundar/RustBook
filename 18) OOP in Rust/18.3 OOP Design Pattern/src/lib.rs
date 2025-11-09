pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post { 
            state: Some(Box::new(Draft {})), 
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // Get the state as a reference, call content() on it.
        self.state.as_ref().unwrap().content(self)
    }
}

// This impl block changes post's state.
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // Extract the current state from the Post. Will temporarily set state to None
            // Set the new state.
            self.state = Some(s.request_review()); // call request_review() on the current state, which will return a new state.
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // Transition from a draft state to a review state. Otherwise, maintain current state.
    fn request_review(self: Box<Self>) -> Box<dyn State>; // Takes ownsership of the old state, consuming it, and returns a new state.
    
     // Transition from a review state to a published state. Otherwise, maintain current state.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // By default, return an empty &str.
    // In the draft and review state this is what will be returned.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    // Consume the current draft state and return a new review state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Maintain current state, as approve() is meant to be used while in review state.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    // Maintain current state, as request_review() is meant to be used while in draft state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Consume the current review state and return a new published state.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    // Maintain current state, as request_review() is meant to be used while in draft state. 
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Maintain current state, as approve() is meant to be used while in review state.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Return the content of the post only in the published state.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}