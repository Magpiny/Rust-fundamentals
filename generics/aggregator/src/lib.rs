// Media aggregator library in Rust

// We want to make a media aggregator library crate named aggregator that can display
// summaries of data that might be stored in a NewsArticle or Tweet instance.
//

//A trait can have multiple methods in its body: the method signatures are listed
//one per line and each line ends in a semicolon.

use std::fmt::Display;
//trait definition
pub trait Sumarry {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementing a trait on a type
impl Sumarry for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

impl Sumarry for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as parameters
pub fn notify(item: impl Sumarry) {
    println!("BREAKING NEWS: {}", item.summarize());
}

// Trait bound syntax
pub fn notifyc<T: Sumarry>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds with + Syntax
pub fn notifyd(item: &(impl Sumarry + Display)) {}

// Returning types that implement traits
fn returns_summarizable() -> impl Sumarry {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// exposing everythng
pub use crate::{Sumarry as Summarry, Tweet as Tweat};
