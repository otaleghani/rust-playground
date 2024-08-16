use design_patterns::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I did something today");
    assert_eq!("", post.content());
    println!("content: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("content: {}", post.content());

    post.approve();
    assert_eq!("I did something today", post.content());
    println!("content: {}", post.content());
}
