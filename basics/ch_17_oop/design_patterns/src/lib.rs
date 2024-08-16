pub struct Post {
    content: String,
    state: Option<Box<dyn State>>
}

impl Post {
    pub fn new() -> Self {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {}))
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // OLD WAY
        self.content.push_str(text);
        //
        // take the state, call the function add_text inside the state
        // instead of here
        //self.state.as_ref().unwrap().add_text(self, text);
        // self.state.as_mut().unwrap().add_text(self, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // Option<>.take() takes the value inside of the Option,
        // leaving None in it's place
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    //fn add_text<'a>(&self, _post: &'a mut Post, text: &str) {
    //    println!("trying to modify when I cannot :(");
    //}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //fn add_text<'a>(&self, post: &'a mut Post, text: &str) {
    //    post.content.push_str(text);
    //}
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
