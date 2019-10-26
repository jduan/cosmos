pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Make America Great Again"),
        location: String::from("Washington DC"),
        author: String::from("Trump"),
        content: String::from("Make America Great Again"),
    };

    println!("1 news article: {}", article.summarize3());

    notify(tweet);
    notify2(article);
}

pub trait Summary {
    fn summarize(&self) -> String;

    // trait can have methods with default implementation
    // this can be overridden by types that implement this trait
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    // Default implementations can call other methods in the same trait, even if those other
    // methods don’t have a default implementation. In this way, a trait can provide a lot of
    // useful functionality and only require implementors to specify a small part of it.
    // This is the "template pattern". The template itself is implemented in the trait while
    // various hooks are implemented by the types themselves.
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits as parameters
// this function can be called with any type that implements Summary
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// "trait bound"
// this is equivalent to the function above, which is actually syntax sugar
fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

trait Display {
    fn show(&self) -> String;
}

// specify multiple traits using +
fn notify3<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
    println!("Show me the item: {}", item.show());
}

// "trait bound" using "where" clause between return type and open curly brace
// this is easier to read when you have many trait bounds
fn some_function<T, U>(t: T, u: U) -> i32
where T: Display + Clone,
      U: Clone + Summary,
{
    99
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
