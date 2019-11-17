// Enums in Rust are basically sum types (ie algebraic data types) in ML languages.
//
// Enums are useful whenever a value might be either one thing or another. The "price" of using
// them is that you must access the data safely, using pattern matching.
//
// Pattern matching is Rust is really powerful. They are a little like regular expressions for all
// your data. They're used to test whether or not a value has a particular desired shape. They can
// extract several fields from a struct or tuple into local variables at all once. And like regular
// expressions, they are concise, typically doing it all in a single line of code.
//
// The Big Picture.
//
// Programming is data processing. Getting data into the right shape can be the difference between
// a small, fast, elegant program and a slow, gigantic tangle of duct tape and virtual method
// calls.
//
// This is the problem space enums address. They are a design tool for getting data into the right
// shape. For cases when a value may be one thing, or another thing, or perhaps nothing at all,
// enums are better than class hierarchies on every axis: faster, safer, less code, easier to
// document.
//
// The limiting factor is flexibility. End users of an enum can’t extend it to add new variants.
// Variants can be added only by changing the enum declaration. And when that happens, existing
// code breaks. Every match expression that individually matches each variant of the enum must be
// revisited—it needs a new arm to handle the new variant. In some cases, revisiting all uses of an
// enum when it changes is exactly what we want.
//
// But sometimes more flexibility is needed. For those situations, Rust has traits.
pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    let msg1 = Message::Quit;
    msg1.call();
    let msg2 = Message::Move { x: 30, y: 50 };
    msg2.call();
    let msg3 = Message::Write(String::from("hello, world"));
    msg3.call();
    let msg4 = Message::ChangeColor(255, 128, 33);
    msg4.call();
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// this is better than the above
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Each variant can have different types
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This enum has a wide variety of types embedded in its variants.
#[derive(Debug)]
enum Message {
    Quit,
    // This uses an anonymous struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can define methods on enums just like structs.
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("This is a Quite message"),
            Message::Move { x: x, y: y } => println!("This is a Move (x: {}, y: {}) message", x, y),
            Message::Write(str) => println!("This is a Write message: {}", str),
            Message::ChangeColor(r, g, b) => {
                println!("This is a ChangeColor({}, {}, {}) message", r, g, b)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

// By default, Rust stores C-style enums using the smallest built-in integer type that an
// accommodate them. Most fit in a single byte. Occasionally, it's useful to tell Rust which
// integers to use.
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[derive(Debug)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(&self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Debug)]
enum RoughTime {
    // This is called a "tuple variant", like "tuple structs".
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    fn to_english(&self) -> String {
        match self {
            RoughTime::InThePast(tu, 1) => format!("a {} ago", tu.singular()),
            RoughTime::InThePast(tu, n) => format!("{} {} ago", n, tu.plural()),
            RoughTime::JustNow => String::from("just now"),
            RoughTime::InTheFuture(tu, 1) => format!("a {} from now", tu.singular()),
            RoughTime::InTheFuture(tu, n) => format!("{} {} from now", n, tu.plural()),
        }
    }
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

enum Shape {
    // These are called "struct variants", similar to "named-field" structs.
    Sphere { center: Point, radius: f32 },
    Cuboid { corner1: Point, corner2: Point },
}

// In all, Rust has parity between structs and enums:
// 1. Variants with no data correspond to unit-like structs
// 2. Tuple variants look and function just like tuple structs
// 3. Struct variants have curly braces and named field, just like regular structs
//
// Note that a single enum can have variants of all three kinds.

// Rich data structures using enums!
//
// Enums are useful for quickly implementing tree-like data structures (recursive), like JSON.
// In memory, any JSON object can be represented as a value of this Rust type:
use std::collections::HashMap;
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    // The Box around the HashMap serves only to make all Json values more compact. All Json values
    // must be the same size (that can fit the largest variant). A HashMap takes eight words or so.
    // But a Box<HashMap> is a single word: it's just a pointer to heap-allocated data.
    Object(Box<HashMap<String, Json>>),
}

// Generic Enums
// Enums can be generic, such as Option<T> and Result<T, E> from the stdlib.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode2<T> {
    element: T,
    left: Option<Box<TreeNode2<T>>>,
    right: Option<Box<TreeNode2<T>>>,
}

struct Account {
    name: String,
    language: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(Ordering::Equal, compare(3, 3));
        assert_eq!(Ordering::Equal, compare(3, 3));
    }

    #[test]
    fn enum_memory_use() {
        use std::mem::size_of;
        assert_eq!(size_of::<Ordering>(), 1);
        assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8
                                                // Casting a C-style enum to an integer is allowed.
        assert_eq!(HttpStatus::Ok as i32, 200);
    }

    #[test]
    fn test_tuple_enums() {
        let past = RoughTime::InThePast(TimeUnit::Years, 1);
        assert_eq!("a year ago", past.to_english());
        let past2 = RoughTime::InThePast(TimeUnit::Years, 2);
        assert_eq!("2 years ago", past2.to_english());
        let future = RoughTime::InTheFuture(TimeUnit::Years, 1);
        assert_eq!("a year from now", future.to_english());
        let future2 = RoughTime::InTheFuture(TimeUnit::Years, 2);
        assert_eq!("2 years from now", future2.to_english());
    }

    #[test]
    fn test_tree_node() {
        let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "Earth",
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));

        match tree {
            BinaryTree::NonEmpty(b) => {
                assert_eq!("Earth", b.element);
            }
            BinaryTree::Empty => panic!("Should never happen."),
        }
    }

    #[test]
    fn cannot_use_existing_variables_when_matching() {
        fn get_lucky_number(n: i32) -> Option<i32> {
            if n > 10 {
                Some(n * 2)
            } else {
                None
            }
        }

        let n = 15;
        match get_lucky_number(n) {
            // Here n shadows the n defined earlier outside.
            Some(n) => assert_eq!(30, n),
            None => panic!("This should never happen."),
        }

        // If your intention is to match Some(n) where n is actually the outside n, you can do
        // this:
        match get_lucky_number(n) {
            Some(num) => {
                if num == n {
                    println!("You're super lucky")
                } else {
                    assert_eq!(30, num);
                }
            }
            None => panic!("This should never happen."),
        }

        // You can also "guard".
        match get_lucky_number(n) {
            Some(num) if num == n => println!("You're super lucky"),
            Some(num) => assert_eq!(30, num),
            None => panic!("This should never happen."),
        }

        assert_eq!(15, n);
    }

    #[test]
    fn shorthand_struct_pattern_match() {
        let p = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        match p {
            Point { x: x, y: y, z: z } => {
                assert_eq!(1.0, x);
                assert_eq!(2.0, y);
                assert_eq!(3.0, z);
            }
        }

        // Patterns like Point { x: x, y: y  } are common when matching structs, and the redundant
        // names are visual clutter, so Rust has a shorthand for this: Point {x, y}. The meaning is
        // the same.
        match p {
            Point { x, y, z } => {
                assert_eq!(1.0, x);
                assert_eq!(2.0, y);
                assert_eq!(3.0, z);
            }
        }
    }

    #[test]
    fn ignore_fields_with_two_dots() {
        let p = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        // It is cumbersome to match a large struct when we only care about a few fields. To avoid
        // this, use .. to tell Rust you don’t care about any of the other fields.
        match p {
            Point { x, .. } => assert_eq!(1.0, x),
        }
    }

    #[test]
    fn pattern_match_refs() {
        let account = Account {
            name: "gold".to_string(),
            language: "en".to_string(),
        };

        match account {
            Account { name, language } => {
                assert_eq!("gold".to_string(), name);
                assert_eq!("en".to_string(), language);

                // Matching on a noncopyable value moves the value. "account" no longer exists
                // here. Here, the fields account.name and account.language are moved into local
                // variables name and language. The rest of account is dropped.
                // assert_eq!("gold".to_string(), account.name);
            }
        }

        let account = Account {
            name: "gold".to_string(),
            language: "en".to_string(),
        };

        match account {
            Account {
                ref name,
                ref language,
            } => {
                assert_eq!("gold", name);
                assert_eq!("en", language);
                // This works now!
                assert_eq!("en".to_string(), account.language);
            }
        }

        let mut account = Account {
            name: "gold".to_string(),
            language: "en".to_string(),
        };

        match account {
            Account {
                ref mut name,
                ref language,
            } => {
                assert_eq!("gold", name);
                name.push_str("!");
                assert_eq!("en", language);
                // This works now!
                assert_eq!("en".to_string(), account.language);
            }
        }
        // name is changed now.
        assert_eq!("gold!", account.name);
    }

    #[test]
    fn ampersand_match() {
        let account: &Account = &Account {
            name: "gold".to_string(),
            language: "en".to_string(),
        };

        match account {
            // A pattern starting with & matches a reference.
            &Account {
                // You need the "ref" here because otherwise it tries to move "name" but you can't
                // move "name" because it's not owned by us. We're only borrowing a ref to account
                // here.
                ref name,
                ref language,
            } => {
                assert_eq!("gold".to_string(), *name);
                assert_eq!("en".to_string(), *language);
            }
        }
    }

    #[test]
    fn match_multiple_possibilities() {
        #[derive(Debug, PartialEq)]
        enum Kind {
            Letter,
            Digit,
            Other,
        }

        fn get_kind(ch: char) -> Kind {
            // The vertical bar (|) can be used to combine several patterns in a single match arm.
            match ch {
                'a'...'z' | 'A'...'Z' => Kind::Letter,
                '0'...'9' => Kind::Digit,
                _ => Kind::Other,
            }
        }

        assert_eq!(Kind::Letter, get_kind('f'));
    }

    #[test]
    fn pattern_guards() {
        let p = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        match p {
            // Use the if keyword to add a guard to a match arm. The match succeeds only if the
            // guard evaluates to true.
            Point { x, y, z } if x == y && x == z => assert_eq!(x, y),
            Point { x, y, z } => assert_eq!(1.0, x),
        }
    }

    #[test]
    fn at_pattern() {
        let msg = Message::ChangeColor(1, 2, 3);

        match msg {
            // Finally, x @ pattern matches exactly like the given pattern, but on success, instead
            // of creating variables for parts of the matched value, it creates a single variable x
            // and moves or copies the whole value into it. Without @ pattern, you will need to
            // manually construct a ChangeColor yourself.
            cc @ Message::ChangeColor(..) => println!("change color is {:?}", cc),
            _ => {} // ignore other kinds
        }
    }
}

// Although patterns are most prominent in match expressions, they are also allowed in several
// other places, typically in place of an identifier.
//
// // ...unpack a struct into three new local variables
// let Track { album, track_number, title, .. } = song;
//
// // ...unpack a function argument that's a tuple
// fn distance_to((x, y): (f64, f64)) -> f64 { ... }
//
// // ...iterate over keys and values of a HashMap
// for (id, document) in &cache_map {
//     println!("Document #{}: {}", id, document.title);
// }
//
// // ...automatically dereference an argument to a closure
// // (handy because sometimes other code passes you a reference
// // when you'd rather have a copy)
// let sum = numbers.fold(0, |a, &num| a + num);
//
//
// Patterns that always match are special in Rust. They’re called irrefutable patterns, and they’re
// the only patterns allowed in the four places shown here (after let, in function arguments, after
// for, and in closure arguments).
//
// A refutable pattern is one that might not match. Refutable patterns can be used in match arms,
// because match is designed for them: if one pattern fails to match, it’s clear what happens next.
// The four examples above are places in Rust programs where a pattern can be handy, but the
// language doesn’t allow for match failure.
//
// Refutable patterns are also allowed in if let and while let expressions.
//
// ...run some code only if a table lookup succeeds
// if let Some(document) = cache_map.get(&id) {
//     return send_cached_response(document);
// }
//
// // ...repeatedly try something until it succeeds
// while let Err(err) = present_cheesy_anti_robot_task() {
//     log_robot_attempt(err);
//     // let the user try again (it might still be a human)
// }
