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
    let pair = Pairz::new(1,2);
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

struct Pairz<T> {
    x: T,
    y: T,
}

impl<T> Pairz<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pairz<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("x is larger. x = {}", self.x);
        } else {
            println!("y is larger. y = {}", self.y);
        }
    }
}
