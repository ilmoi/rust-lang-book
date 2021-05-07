fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve(); //pending post
    let second_post;
    if let Posts::PP(x) = post {
        second_post = x.approve(); //readypost
        if let Posts::RP(x) = second_post {
            println!("Txt: {}", x.content());
            println!("Txt2: {}", x.content2());
        }
    }
}

// -----------------------------------------------------------------------------
// post

struct Post {}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::from(""),
        }
    }
}

// -----------------------------------------------------------------------------
// draft

struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) { //takes &mut self because we're updating our own prop
        self.content = String::from(text);
    }
    pub fn request_review(self) -> PendingPost { //takes just self because we're actually MOVING content to a new owner
        PendingPost {
            content: self.content,
            approvals: 0,
        }
    }
}

// -----------------------------------------------------------------------------
// pending

struct PendingPost {
    content: String,
    approvals: u32,
}

enum Posts {
    PP(PendingPost),
    RP(ReadyPost),
}

impl PendingPost {
    fn approve(mut self) -> Posts { //same reason - we're actually MOVING content, so takes just "self", without &
        self.approvals += 1;
        if self.approvals >= 2 { //ready post
            Posts::RP(ReadyPost {
                content: self.content,
            })
        } else { //pending post
            Posts::PP(self)
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

// -----------------------------------------------------------------------------0
// ready

struct ReadyPost {
    content: String,
}

impl ReadyPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}

// -----------------------------------------------------------------------------
// trait

//testing what implementing a trait on all would look like
trait GetContent {
    fn content2(&self) -> &str; //can't have &self.content inside a trait, trait has no props
}

impl GetContent for ReadyPost {
    fn content2(&self) -> &str {
        &self.content //but when we re-implement a trait then sure we can reference it
    }
}
