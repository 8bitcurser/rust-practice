pub struct Post {
    content: String
}

pub struct DraftPost {
    content: String
}


impl Post {
    // a new post generates a new draft post
    // content is accesible but when the instance is post meaning, when it 
    // was approved after converting from draft to pending to post again
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
    // a draft can have data added to itself
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // a draft can be converted into a PendingReviewPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
           content: self.content,
           approvals: 0
        }
    }
}


pub struct PendingReviewPost {
    content: String,
    approvals: u32
}


impl PendingReviewPost {
    // a pending review post can be approved and turned into a post
    pub fn approve(&mut self) {
        self.approvals += 1;
    }

    pub fn publish(self) -> Option<Post> {
        if self.approvals > 1 {
           Some(Post {
                content: self.content
            })
        } else {
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}
