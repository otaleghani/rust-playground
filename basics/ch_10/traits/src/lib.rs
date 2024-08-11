use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more from {}",
            self.summarize_author()
        )
    }
    fn summarize_author(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})",
    //        self.headline,
    //        self.author,
    //        self.location
    //    )
    //}
    fn summarize_author(&self) -> String {
        format!("the author {}",
            self.author
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",
            self.username,
            self.content
        )
    }
    fn summarize_author(&self) -> String {
        format!("{}, also known as the dickhead",
            self.username
        )
    }
}

// a method that implements whatever type implements Summary
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Another ethod that implenents whatever type implements Summary
// but in a more verbose way (called trait bound)
pub fn notify_again<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// This second one is needed if we want to specify two items
// having the same type
pub fn notify_two<T: Summary>(item_one: &T, item_two: &T) {
    println!("{}, {}", item_one.summarize(), item_two.summarize());
}

// we could even have methods that need that one item implements
// more than one trait
pub fn test_both<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize());
}
pub fn test_both_two(item: &(impl Summary + Display)) {
    println!("{}", item.summarize());
}

// For more clutter signature this actually becomes quite a shitty way
// of writing a signature. For example this is really hard to read
// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// For this reason rust has the Where clause that could make it a little better
// pub fn some_other_fn<T, U>(t: &T, u: &U) -> i32 
// where 
//     T: Display + Clone,
//     U: Clone + Debug,
// {}


