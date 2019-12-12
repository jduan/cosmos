/// One of the great discoveries in programming is that it’s possible to write code that operates on
/// values of many different types, even types that haven’t been invented yet.
///
/// It’s called polymorphism.
///
/// Rust supports polymorphism with two related features: traits and generics. These concepts will
/// be familiar to many programmers, but Rust takes a fresh approach inspired by Haskell’s
/// typeclasses.
///
/// Generics and traits are closely related. For example, you can write a function to compare two
/// values and find the smaller one. The function signature would looke like this:
///      fn min<T: Ord>(value1: T, value2: T) -> T
///
/// This function works with any type T that implements the Ord trait.
///
/// Using Traits
///
/// A trait is a feature that any given type may or may not support. Most often, a trait represents
/// a capability: something a type can do.
///
///     A value that implements std::io::Write can write out bytes.
///
///     A value that implements std::iter::Iterator can produce a sequence of values.
///
///     A value that implements std::clone::Clone can make clones of itself in memory.
///
///     A value that implements std::fmt::Debug can be printed using println!() with the
///     {:?} format specifier.
///
/// There is one unusual rule about trait methods: the trait itself must be in scope. Otherwise,
/// all its methods are hidden.
///
/// Rust has this rule because, as we’ll see later in this chapter, you can use traits to add new
/// methods to any type—even standard library types like u32 and str. Third-party crates can do the
/// same thing. Clearly, this could lead to naming conflicts! But since Rust makes you import the
/// traits you plan to use, crates are free to take advantage of this superpower, and conflicts are
/// rare in practice.
///
/// The reason Clone and Iterator methods work without any special imports is that they’re always
/// in scope by default: they’re part of the standard prelude, names that Rust automatically
/// imports into every module. In fact, the prelude is mostly a carefully chosen selection of
/// traits.
///
///
/// ## when to use which (trait objects vs generic functions)
///
/// Both features are based on traits. They have a lot in common but there are subtle differences.
///
/// 1. Trait objects are the right choice when you need a collection of values of mixed types, all together.
///
/// trait Vegetable {...}
///
/// struct Salad<V: Vegetable> {
///   veggies: Vec<V>
/// }
///
/// This works but each such salad consists entirely of a single type of vegetable.
///
/// struct Salad {
///     veggies: Vec<Vegetable>  // error: `Vegetable` does not have
///                              //        a constant size
/// }
///
///
/// struct Salad {
///     veggies: Vec<Box<Vegetable>>
/// }
///
/// This code works because each Box<Vegetable> can own any type of vegetable, but the box itself
/// has a constant size—two pointers—suitable for storing in a vector.
///
/// 2. Another possible reason to use trait objects is to reduce the total amount of compiled code.
/// Rust may have to compile a generic function many times, once for each type it’s used with.
/// This could make the binary large, a phenomenon called code bloat in C++ circles.
///
/// ### when to use generic functions
///
/// Generics have two important advantages over trait objects, with the result that in Rust,
/// generics are the more common choice.
///
/// 1. The first advantage is speed. Each time the Rust compiler generates machine code for a
/// generic function, it knows which types it’s working with, so it knows at that time which
/// write method to call. There’s no need for dynamic dispatch.
///
/// Compare that to the behavior with trait objects. Rust never knows what type of value a trait
/// object points to until run time.
///
/// 2. The second advantage of generics is that not every trait can support trait objects. Traits
/// support several features, such as static methods, that work only with generics: they rule out
/// trait objects entirely.
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::io::Write;

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

impl NewsArticle {
    // You can't define this function in the "impl Summary for NewsArticle" block
    // because it's not a function of the NewsArticle trait!
    fn get_headline(&self) -> &String {
        &self.headline
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
where
    T: Display + Clone,
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

// This is a plain function that takes a "trait object".
fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// In contrast, this is a generic function whose type parameter W is bound by "Write" trait.
fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n");
    out.flush()
}

// Find the top occurring elements from a vector.
// This is how to special a type parameter that implements multiple traits.
fn top_ten<T>(values: &Vec<T>) -> Vec<&T>
where
    T: Debug + Hash + Eq,
{
    let mut map = HashMap::new();
    for value in values {
        let counter = map.entry(value).or_insert(0);
        *counter += 1;
    }

    let mut map_vec: Vec<_> = map.into_iter().collect();
    map_vec.sort_by(|a, b| b.1.cmp(&a.1));
    map_vec.into_iter().map(|a| a.0).take(10).collect()
}

trait Mapper {}
trait Reducer {}
trait Serialize {}
struct DataSet {}
// Generic functions can have multiple type parameters: M and R.
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: R) {
    unimplemented!()
}

// Alternative syntax: bounds can be specified in the where clause
fn run_query2<M, R>(data: &DataSet, map: M, reduce: R)
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
    unimplemented!()
}

trait MeasureDistance {}
// A generic function can have both lifetime parameters and type parameters. Lifetime parameters
// come first.
fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
where
    P: MeasureDistance,
{
    unimplemented!()
}

/// This is a generic function. It works with parameters that implement the "Ord" trait.
/// The compiler generates custom machine code for each type T that you actually use.
fn min<T: Ord>(m: T, n: T) -> T {
    if m < n {
        m
    } else {
        n
    }
}

/// Rust lets you implement any trait on any type, as long as either the trait or the type is
/// introduced in the current crate. This means that any time you want to add a method to any type,
/// you can use a trait to do it. This is called an "extension trait".
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        unimplemented!()
    }
}

/// We said earlier that when you implement a trait, either the trait or the type must be new in
/// the current crate. This is called the "coherence rule". It helps Rust ensure that trait
/// implementations are unique. Your code can’t "impl Write for u8", because both Write and u8 are
/// defined in the standard library. If Rust let crates do that, there could be multiple
/// implementations of Write for u8, in different crates, and Rust would have no reasonable way to
/// decide which implementation to use for a given method call.

///You can even use a generic impl block to add an extension trait to a whole family of types at once.
struct HtmlDocument {}
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

/// You can write HTML to any std::io writer.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        unimplemented!()
    }
}

/// Self in traits
///
/// A trait can use the keyword Self as a type. It represents the trait itself.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(min(3, 5), 3);
        assert_eq!(min(30, 5), 5);
    }

    #[test]
    fn traits_need_to_be_in_scope() {
        // The Write trait needs to be in scope. Otherwise, all its methods (such as "write_all")
        // are hidden.
        use std::io::Write;

        let mut buf: Vec<u8> = vec![];
        buf.write_all(b"hello").unwrap();
        assert_eq!(5, buf.len());

        // Note that only Vec<u8> implements the Write trait. So the code below doesn't work!
        //        let mut buf: Vec<String> = vec![];
        //        buf.write_all("hello world").unwrap();
        //        assert_eq!(11, buf.len());
    }

    #[test]
    fn trait_objects() {
        let mut buf: Vec<u8> = vec![];

        // Rust doesn’t permit variables of type Write!
        // This line doesn't compile because a variable's size has to be known at compile time and
        // types that implement Write can be any size.
        // let writer: Write = buf;

        // A reference to a trait type, like writer, is a called a "trait object". Like any other
        // reference, a trait object points to some value, it has a lifetime, and it can be either
        // mut or shared. The size of a reference is fixed!
        let writer: &mut Write = &mut buf;

        // What makes a trait object different is that Rust usually doesn’t know the type of the
        // referent at compile time. So a trait object includes a little extra information about
        // the referent’s type. This is strictly for Rust’s own use behind the scenes: when you
        // call writer.write(data), Rust needs the type information to dynamically call the right
        // write method depending on the type of *writer. You can’t query the type information
        // directly, and Rust does not support downcasting from the trait object &mut Write back to
        // a concrete type like Vec<u8>. In other words, you can only work with the "generic type"
        // of the trait itself.
        //
        // In memory, a trait object is a "fat pointer" consisting of a pointer to the value, plus
        // a pointer to a table representing that value's type. (Vec<u8> in this example)

        // Rust automatically converts ordinary references into trait objects when needed. Let's
        // say "say_hello" is a function that takes a "&mut Write", this works:
        //
        // let mut local_file: File = File::create("hello.txt")?;
        // say_hello(&mut local_file)?; // Rust converts "&mut File" to "&mut Write"

        // This kind of conversion is the only way to create a trait object. What the computer is
        // actually doing here is very simple. At the point where the conversion happens, Rust
        // knows the referent’s true type (in this case, File), so it just adds the address of the
        // appropriate vtable, turning the regular pointer into a fat pointer.
    }

    #[test]
    fn test_generic_functions() {}

    #[test]
    fn test_top_ten() {
        let names = vec![
            String::from("Oakland"),
            String::from("Oakland"),
            String::from("Oakland"),
            String::from("Alameda"),
            String::from("San Francisco"),
            String::from("San Francisco"),
        ];
        let top10 = top_ten(&names);
        assert_eq!(vec!["Oakland", "San Francisco", "Alameda"], top10);
    }
}
