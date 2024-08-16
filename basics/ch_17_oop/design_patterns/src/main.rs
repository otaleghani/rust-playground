use design_patterns::type_one::Post;
use design_patterns::type_two::PostTwo;

fn main() {
    type_one();
    type_two();
}

fn type_one() {
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

fn type_two() {
    let mut post = PostTwo::new();
    post.add_text("anvedi come balla nando");
    //println!("content: {}", post.content());

    let post = post.request_review();
    //println!("content: {}", post.content());

    let post = post.approve();
    println!("content: {}", post.content());
}
