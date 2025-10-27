use blog::{Post, PostWithEnum};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post2 = PostWithEnum::new();

    post2.add_text("I ate a salad for lunch today");
    assert_eq!("", post2.content());

    post2.request_review();
    assert_eq!("", post2.content());

    post2.approve();
    assert_eq!("I ate a salad for lunch today", post2.content());
}
