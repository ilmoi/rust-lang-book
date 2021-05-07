use std::boxed::Box;

fn main() {
    let mut post = Post::new();

    //Draft
    post.add_text("yaba daba du");
    post.start_the_piece("hello from draft stage"); //changes
    assert_eq!(post.content(), "empty!");

    post.request_review();
    assert_eq!(post.content(), "empty!");

    // post.reject();
    post.approve();
    post.approve();
    // assert_eq!(post.content(), "yaba daba du");

    // post.add_text("yaba daba du22222222222");
    post.start_the_piece("hello from later stage"); //doesn't = good

    println!("{}", post.content());
    println!("{:?}", post.state);
}

// -----------------------------------------------------------------------------
// Post

struct Post {
    contents: String,
    state: Option<Box<dyn Publishable>>,
    //NOTE: we're using option here because we need to be able to call .take() later in the code
    approvals: u32,
}

impl Post {
    //NOTE: fns = public
    pub fn new() -> Self {
        Self {
            contents: String::from(""),
            state: Some(Box::new(Draft {})),
            approvals: 0,
        }
    }
    pub fn add_text(&mut self, new_content: &str) {
        self.contents = String::from(new_content);
    }

    // the next fns delegate logic to Publishable states to handle
    pub fn start_the_piece(&mut self, new_string: &str) {
        let chosen_string = self.state.as_ref().unwrap().start_the_piece(&self.contents[..], new_string);
        self.contents = String::from(chosen_string);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self) //have to use as_ref because we're using Option<>
    }
    pub fn request_review(&mut self) {
        // naive way of writing it
        // let cur_state = self.state.take(); //NOTE: can only do because we wrapped Box in an Option. Otherwise the take() method would not be available
        // let new_state = cur_state.unwrap().request_review();
        // self.state = Some(new_state);

        // better way of writing it
        // NOTE1: we need take() here because we need to pass the actual state value, not a ref, for it to be consumed below
        // NOTE2: take() lets us do that - it pulls the Some() value out of the option, leaving None in place. Rem that rust doens't let us have null values, so this is our way of temporarily having one
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()); //consumes current state and returns a new state
        }
    }
    pub fn approve(&mut self) {
        self.approvals += 1;

        if self.approvals >= 2 {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

// -----------------------------------------------------------------------------
// trait

// what we're doing here with std::fmt::Debug is we're having our trait rely on another trait - https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html
// but this only tells the compiler that we must have Debug implemented in our structs - which we then do with #[derive(Debug)] below
trait Publishable: std::fmt::Debug {
    //NOTE: fns = private
    fn content<'a>(&self, post: &'a Post) -> &'a str { "empty!" }
    fn start_the_piece<'a>(&self, old_string: &'a str, _: &'a str) -> &'a str { old_string }
    //has a default unlike next 2
    //NOTE1: self: Box<Self> syntax means that this method will only be implemented when the holding type is Box
    //NOTE2: I tried passing &self instead of self. That breaks the return value. To return an actual object and not a ref, we also need to pass an actual object to self and not a ref.
    //NOTE3: thus in effect the fn below takes ownership of Box<Self>
    fn request_review(self: Box<Self>) -> Box<dyn Publishable>;
    fn approve(self: Box<Self>) -> Box<dyn Publishable>;
    fn reject(self: Box<Self>) -> Box<dyn Publishable>;
}

// -----------------------------------------------------------------------------
// states

#[derive(Debug)] //must have this as well as : std::fmt::Debug on trait
struct Draft {}

impl Publishable for Draft {
    fn start_the_piece<'a>(&self, _: &'a str, new_string: &'a str) -> &'a str { new_string }
    fn request_review(self: Box<Self>) -> Box<dyn Publishable> { Box::new(PendingReview {}) }
    fn approve(self: Box<Self>) -> Box<dyn Publishable> { self }
    fn reject(self: Box<Self>) -> Box<dyn Publishable> { self }
}

#[derive(Debug)] //must have this as well as : std::fmt::Debug on trait
struct PendingReview {}

impl Publishable for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn Publishable> { self }
    //no, this can't be a default in the trait, coz trait doesn't know the size of "self" at compile time
    fn approve(self: Box<Self>) -> Box<dyn Publishable> { Box::new(ReadyPost {}) }
    fn reject(self: Box<Self>) -> Box<dyn Publishable> { Box::new(Draft {}) }
}

#[derive(Debug)] //must have this as well as : std::fmt::Debug on trait
struct ReadyPost {}

impl Publishable for ReadyPost {
    fn content<'a>(&self, post: &'a Post) -> &'a str { &post.contents }
    fn request_review(self: Box<Self>) -> Box<dyn Publishable> { self }
    //no, this can't be a default in the trait, coz trait doesn't know the size of "self" at compile time
    fn approve(self: Box<Self>) -> Box<dyn Publishable> { self }
    fn reject(self: Box<Self>) -> Box<dyn Publishable> { self }
}