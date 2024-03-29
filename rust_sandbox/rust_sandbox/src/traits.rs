/// One of the great discoveries in programming is that it’s possible to write code that operates on
/// values of many different types, even types that haven’t been invented yet.
///
/// It’s called "polymorphism".
///
/// # Traits and Generics
///
/// Rust supports polymorphism with two related features: traits and generics. These concepts will
/// be familiar to many programmers, but Rust takes a fresh approach inspired by Haskell’s
/// typeclasses.
///
/// Generics and traits are closely related. For example, you can write a function to compare two
/// values and find the smaller one. The function signature would looke like this:
///
///      fn min<T: Ord>(value1: T, value2: T) -> T
///
/// This function works with any type T that implements the Ord trait.
///
/// # Using Traits
///
/// A trait is a feature that any given type may or may not support. Most often, a trait represents
/// a "capability": something a type can do.
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
/// * dynamic dispatch: trait objects
/// * static dispatch: generic functions with trait bounds
///
/// How to understand "trait object"? Trait objects are very similar to how Java does dynamic
/// dispatch, ie "polymorphism". In Java, you can have references that point to various subtypes of
/// an interface. When you call methods on the reference, depending on the concrete subtype, a
/// different implemention may get invoked. That's called "dynamic dispatch". Trait objects are
/// equivalent to those references in Java and you can use "trait objects" to do dynamic dispatch.
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
/// write method to call. This is called "static dispatch", in contrast to "dynamic dispatch".
///
/// Compare that to the behavior with trait objects. Rust never knows what type of value a trait
/// object points to until run time.
///
/// 2. The second advantage of generics is that not every trait can support trait objects. Traits
/// support several features, such as static methods, that work only with generics: they rule out
/// trait objects entirely.
///
/// You can only make "object-safe traits" into trait objects. Some complex rules govern all the
/// properties that make a trait object safe, but in practice, only two rules are relevant. A
/// trait is object safe if all the methods defined in the trait have the following properties:
///
///  * The return type isn’t Self.
///  * There are no generic type parameters.
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::ops::Mul;

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
    pub fn get_headline(&self) -> &String {
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
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// "trait bound"
// this is equivalent to the function above, which is actually syntax sugar
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub trait Display {
    fn show(&self) -> String;
}

// specify multiple traits using +
pub fn notify3<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
    println!("Show me the item: {}", item.show());
}

// "trait bound" using "where" clause between return type and open curly brace
// this is easier to read when you have many trait bounds
pub fn some_function<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    99
}

// returning types that implement traits
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// This is a plain function that takes a "trait object".
pub fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// In contrast, this is a generic function whose type parameter W is bound by "Write" trait.
pub fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// Find the top occurring elements from a vector.
// This is how to special a type parameter that implements multiple traits.
pub fn top_ten<T>(values: &[T]) -> Vec<&T>
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

pub trait Mapper {}
pub trait Reducer {}
pub trait Serialize {}
pub struct DataSet {}
// Generic functions can have multiple type parameters: M and R.
pub fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    _data: &DataSet,
    _map: M,
    _reduce: R,
) {
    unimplemented!()
}

// Alternative syntax: bounds can be specified in the where clause
pub fn run_query2<M, R>(_data: &DataSet, _map: M, _reduce: R)
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
    unimplemented!()
}

pub trait MeasureDistance {}
// A generic function can have both lifetime parameters and type parameters. Lifetime parameters
// come first.
pub fn nearest<'t, 'c, P>(_target: &'t P, _candidates: &'c [P]) -> &'c P
where
    P: MeasureDistance,
{
    unimplemented!()
}

/// This is a generic function. It works with parameters that implement the "Ord" trait.
/// The compiler generates custom machine code for each type T that you actually use.
pub fn min<T: Ord>(m: T, n: T) -> T {
    if m < n {
        m
    } else {
        n
    }
}

/// Rust lets you implement any trait on any type, as long as either the trait or the type is
/// introduced in the current crate. This means that any time you want to add a method to any type,
/// you can use a trait to do it. This is called an "extension trait".
pub trait IsEmoji {
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
pub struct HtmlDocument {}
pub trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

/// You can write HTML to any std::io writer.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, _html: &HtmlDocument) -> io::Result<()> {
        unimplemented!()
    }
}

/// Self in traits
///
/// A trait can use the keyword Self as a type. It represents the trait itself.
pub trait MyClone {
    fn clone(&self) -> Self;
}

/// Subtraits: we can define a trait is an extension of another trait
/// This means that every type that implements Creature must also implement the Display trait.
pub trait Creature: Display {
    fn position(&self) -> (i32, i32);
}
// impl Display for Broom {}
// impl Creature for Broom {}

pub trait Animal {
    // Instance methods
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default implementation.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

pub struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    pub fn shear(&mut self) {
        if self.is_naked() {
            // You can call the trait method "name()" here because Sheep implements
            // the Animal trait.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaa?"
        } else {
            "baaaaaa!"
        }
    }

    // Default implementation can be overridden.
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

/// The compiler is capable of providing basic implementations for some traits via
/// the #[derive] attribute. The following is a list of derivable traits:
///
/// * Comparison traits: Eq, PartialEq, Ord, PartialOrd.
/// * Clone: to create T from &T via a copy.
/// * Copy: to give a type 'copy semantics' instead of 'move semantics'.
/// * Hash: to compute a hash from &T.
/// * Default: to create an empty instance of a data type.
/// * Debug: to format a value using the {:?} formatter.

/// Returning Traits with "dyn"
///
/// https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html
///
/// The Rust compiler needs to know how much space every function's return type requires. This
/// means all your functions have to return a concrete type. Unlike other languages, if you have a
/// trait like Animal, you can't write a function that returns Animal, because its different
/// implementations will need different amounts of memory.
///
/// However, there's an easy workaround. Instead of returning a trait object directly, our
/// functions return a Box which contains some Animal. A box is just a reference to some memory in
/// the heap. Because a reference has a statically-known size, and the compiler can guarantee it
/// points to a heap-allocated Animal, we can return a trait from our function!
///
/// Rust tries to be as explicit as possible whenever it allocates memory on the heap. So if your
/// function returns a pointer-to-trait-on-heap in this way, you need to write the return type with
/// the dyn keyword, e.g. Box<dyn Animal>.

pub struct Cow {}

impl Animal for Cow {
    fn name(&self) -> &'static str {
        "Dave"
    }

    fn noise(&self) -> &'static str {
        "Moo"
    }
}

pub fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {
            name: "Bob",
            naked: true,
        })
    } else {
        Box::new(Cow {})
    }
}

/// Operator overloading
/// https://doc.rust-lang.org/core/ops/
/// In Rust, many of the operators can be overloaded via traits. That is, some operators can be used to accomplish different tasks based on their input arguments. This is possible because operators are syntactic sugar for method calls. For example, the + operator in a + b calls the add method (as in a.add(b)). This add method is part of the Add trait. Hence, the + operator can be used by any implementor of the Add trait.

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Mul<u32> for Rectangle {
    type Output = Self;

    fn mul(self, times: u32) -> Self::Output {
        Rectangle {
            width: self.width * times,
            height: self.height * times,
        }
    }
}

/// impl Trait
/// If your function returns a type that implements MyTrait, you can write its return
/// type as -> impl MyTrait. This can help simplify your type signatures quite a lot!

pub fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    // You could also write the following which is a lot more complicated.
    //    -> std::iter::Chain<std::vec::IntoIter<i32>, std::vec::IntoIter<i32>> {
    v.into_iter().chain(u.into_iter())
}

/// More importantly, some Rust types can't be written out. For example, every
/// closure has its own unnamed concrete type. Before impl Trait syntax, you had
/// to allocate on the heap in order to return a closure. But now you can do it
/// all statically, like this:
pub fn make_adder(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

/// Polymorphism via trait objects
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Default for Screen {
    fn default() -> Self {
        Screen { components: vec![] }
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn add_component(&mut self, draw: Box<dyn Draw>) -> &mut Self {
        self.components.push(draw);
        self
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a {:?}", self);
    }
}

/// Implement the "state pattern" via trait objects.
trait State {
    // "self: Box<Self>" means that the method is only valid when called on a Box holding the type.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

/// Now we can start seeing the advantages of the state pattern: the request_review method on
/// Post is the same no matter its state value. Each state is responsible for its own rules.
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content.as_ref()
    }
}

pub struct Post {
    /// To consume the old state, the request_review method needs to take ownership of the state
    /// value. This is where the Option in the state field of Post comes in: we call the take
    /// method to take the Some value out of the state field and leave a None in its place,
    /// because Rust doesn’t let us have unpopulated fields in structs. This lets us move the
    /// state value out of Post rather than borrowing it. Then we’ll set the post’s state value
    /// to the result of this operation.
    state: Option<Box<dyn State>>,
    content: String,
}

/// Post knows nothing about the various behaviors. It replies on various State objects to do
/// their jobs.
impl Default for Post {
    fn default() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

impl Post {
    // This behavior doesn’t depend on the state the post is in, so it’s not part of the state
    // pattern. The add_text method doesn’t interact with the state field at all, but it is part
    // of the behavior we want to support.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match &self.state {
            Some(s) => s.content(self),
            None => "",
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

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
        let _writer: &mut dyn Write = &mut buf;

        // What makes a trait object different is that Rust usually doesn’t know the type of the
        // referent at compile time. So a trait object includes a little extra information about
        // the referent’s type. This is strictly for Rust’s own use behind the scenes: when you
        // call writer.write(data), Rust needs the type information to dynamically call the right
        // write method depending on the type of *writer. You can’t query the type information
        // directly, and Rust does not support downcasting from the trait object &mut Write back to
        // a concrete type like Vec<u8>. In other words, you can only work with the "generic type"
        // of the trait itself.
        //
        // In memory, a trait object is a "fat pointer" consisting of two pointers:
        // 1. data pointer: a pointer to the value, plus
        // 2. vtable pointer: a pointer to a table representing that value's type.
        // (Vec<u8> in this example)
        //
        // A vtable is essentially a struct of function pointers, pointing to the concrete piece of
        // machine code for each method in the implementation. A method call like
        // trait_object.method() will retrieve the correct pointer out of the vtable and then do a
        // dynamic call of it.

        // Rust automatically converts ordinary references into trait objects when needed. Let's
        // say "say_hello" is a function that takes a "&mut Write", this works:
        //
        // let mut local_file: File = File::create("hello.txt")?;
        // say_hello(&mut local_file)?; // Rust converts "&mut File" to "&mut Write"

        // This kind of conversion is the only way to create a trait object. What the computer is
        // actually doing here is very simple. At the point where the conversion happens, Rust
        // knows the referent’s true type (in this case, File), so it just adds the address of the
        // appropriate "vtable", turning the regular pointer into a fat pointer.
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

    #[test]
    fn test_sheep() {
        let mut sheep = Sheep {
            naked: true,
            name: "Dolly",
        };
        sheep.talk();
        sheep.shear();
        sheep.talk();
    }

    #[test]
    fn return_trait_object() {
        let animal = random_animal(0.3);
        assert_eq!("baaaaaa?", animal.noise());
    }

    #[test]
    fn operator_overloading() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        let rect2 = rect * 10;
        assert_eq!(100, rect2.width);
        assert_eq!(200, rect2.height);
    }

    #[test]
    fn test_combine_vecs() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let mut v3 = combine_vecs(v1, v2);
        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());
        assert_eq!(None, v3.next());
    }

    #[test]
    fn test_make_adder() {
        let add_one = make_adder(1);
        assert_eq!(2, add_one(1));
    }

    #[test]
    fn test_screen() {
        let mut screen = Screen::default();
        screen
            .add_component(Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }))
            .add_component(Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }));

        screen.run();
    }

    #[test]
    fn test_post() {
        let mut post = Post::default();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
