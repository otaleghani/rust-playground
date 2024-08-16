use traits::{Summary, NewsArticle, Tweet, notify, notify_again, notify_two};
use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: "some".to_string(),
        content: "thin".to_string(),
        reply: false,
        retweet: false
    };
    let tweet_two = Tweet {
        username: "anvedi".to_string(),
        content: "come balla nando".to_string(),
        reply: false,
        retweet: false
    };
    let news_article = NewsArticle {
        headline: "media fucking sucks".to_string(),
        location: "everywhere".to_string(),
        author: "everyfuckingone".to_string(),
        content: "the content".to_string(),
    };

    println!("{}", tweet.summarize());
    println!("{}", news_article.summarize());
    notify(&tweet);
    notify_again(&tweet);
    notify_two(&tweet, &tweet_two);
    println!("{:?}", return_content().summarize());
    let pair = Pair::new(1,2);
    pair.cmp_display();
}

// returning types that implements trait. 
fn return_content() -> impl Summary {
    Tweet {
        username: "anvedi".to_string(),
        content: "come balla nando".to_string(),
        reply: false,
        retweet: false
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// blanket implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("x is larger. x = {}", self.x);
        } else {
            println!("y is larger. y = {}", self.y);
        }
    }
}

// This kind of implementation is widely used in the std 
// library. Just think about the to_string method that is
// present on every type that implements the Display trait.
// This is because there is a blanket impl that wraps every
// type that impl Display with the ToString trait, like this
// impl<T: Display> ToString for T {}
