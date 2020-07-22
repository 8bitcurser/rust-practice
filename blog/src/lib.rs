pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // as_ref on the Option because we want the value inside the Option
        // rather than ownership over it
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        // this method consumes the previous state and provides a new one
        // take retrieves the Some value out of the state field and replace it
        // with None (this is temporary) this way we still can use the old 
        // state. Clearer is like this with take we grab the state and put a
        // None temporarily while we obtain the new state but to do so we need
        // to use the previous state which now we control (s.request_review())
        // and set it to the field.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        // we do the same, we TAKE the inner state, replace it temporarily with 
        // none (we claim its ownership) and we use the previous state to 
        // obtain a new one.
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

}

trait State {
    // self: Box<Self> means the method is only valid if calling a box holding 
    // type, without the &, we now OWN self and need to provide a new one
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // we consume the Draft state and provide the one of Pending Review
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // it returns itself
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // it returns itself
        self
    }

    fn approve(self: Box<Self) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

}
