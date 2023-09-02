pub struct Post {
    content1: String,
}

pub struct DraftPost {
    mcontent: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            mcontent: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content1
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.mcontent.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.mcontent,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content1: self.content,
        }
    }
}
