use std::cmp::PartialEq;

// The author mentions that with enum we need to constantly check the value
// with a match expression. The PostWithEnum is an implementation to check the
// differences. The usage, as shown in main.rs is exactly the same.

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
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

enum EnumState {
    Draft,
    Review,
    Published,
}

pub struct PostWithEnum {
    state: EnumState,
    content: String,
}

impl PartialEq for EnumState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl PostWithEnum {
    pub fn new() -> PostWithEnum {
        PostWithEnum {
            state: EnumState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self: &mut PostWithEnum) {
        match self.state {
            EnumState::Draft => {
                self.state = EnumState::Review;
            }
            EnumState::Review => {}
            EnumState::Published => {}
        }
    }

    pub fn approve(self: &mut PostWithEnum) {
        match self.state {
            EnumState::Draft => {}
            EnumState::Review => {
                self.state = EnumState::Published;
            }
            EnumState::Published => {}
        }
    }

    pub fn content(&self) -> &str {
        if self.state == EnumState::Published {
            return &self.content;
        }

        ""
    }
}
