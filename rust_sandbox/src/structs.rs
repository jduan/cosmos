pub fn run() {
    // You don't need to specify the keys in the same order
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user's email is {}", user.email);
    user.email = String::from("john@example.com");
    println!("user's email is {}", user.email);

    let user2 = create_user(String::from("Jack@airbnb.com"), String::from("jack"));
    println!("user2's email is {}", user2.email);

    let user3 = update_struct(user2);
    println!("user3's email is {}", user3.email);

    let mut user4 = User {
        username: String::from("Greg"),
        ..user
    };
    println!("user4's email is {}", user4.email);
    user4.email.push_str("!!!");
    println!("user4's updated email is {}", user4.email);
    // The line below won't compile because "user.email" has been moved to
    // "user4.email".
    // println!("user's email is {}", user.email);

    // This still works because user4 has its own username.
    println!("user's username is {}", user.username);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rectangle {:#?} is {}", rect, area(&rect));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// "tuple structs"
// Tuple structs have the added meaning the struct name provides but don’t have names associated
// with their fields. Tuple structs are useful when you want to give the whole tuple a name and
// make the tuple be a different type from other tuples, and naming each field as in a regular
// struct would be verbose or redundant.
struct Color(i32, i32, i32);
// fields are private by default
struct Bounds(pub usize, pub usize);

fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

fn update_struct(user: User) -> User {
    User {
        email: String::from("joel@airbnb.com"),
        username: String::from("Joel"),
        // the rest fields come from "user"
        ..user
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// T is a "type parameter"
pub struct Queue<T> {
    items: Vec<T>,
}

// Methods are also known as "associated functions", since they’re associated with
// a specific type. The opposite of an associated function is a "free function",
// one that is not defined as an impl block’s item.
impl<T> Queue<T> {
    // You can also define methods that don’t take self as an argument at all.
    // These become functions associated with the struct type itself, not with
    // any specific value of the type. They're called "static methods".
    //
    // It’s conventional in Rust for constructor functions to be named new.
    // "new" isn't a keyword, and types often have other static methods that
    // serve as constructors, like Vec::with_capacity.
    fn new() -> Queue<T> {
        // Note that we didn't have to write the type parameter T here. This is
        // Rust’s type inference at work: since there’s only one type that works
        // for that function’s return value—namely, Queue<T>—Rust supplies the
        // parameter for us.
        Queue { items: Vec::new() }
    }

    // Shorthand: writing out Queue<T> everywhere becomes a mouthful and a
    // distraction. As another shorthand, every impl block, generic or not,
    // defines the special type parameter Self (note the CamelCase name) to be
    // whatever type we’re adding methods to. Here: "Self" would be Queue<T>.
    fn with_capacity(capacity: usize) -> Self {
        Queue {
            items: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Structs with lifetime parameters
// If a struct contains references, you must name those references' lifetimes.

struct Extrema<'a> {
    greatest: &'a i32,
    least: &'a i32,
}

// Accordding "elision rule", you can omit the lifetime parameter for this
// function.
// fn find_extrema<'a>(slice: &'a [i32]) -> Extrema<'a> {
fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        let item = &slice[i];
        if *item > *greatest {
            greatest = item;
        }
        if *item < *least {
            least = item;
        }
    }

    Extrema {
        greatest: greatest,
        least: least,
    }
}

// Deriving Common Traits for Struct Types
// Each of these traits can be implemented automatically for a struct, provided
// that each of its fields implements the trait.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_structs() {
        let red = Color(255, 0, 0);
        assert_eq!(255, red.0);
        assert_eq!(0, red.1);
    }

    #[test]
    fn test_queue() {
        // Note that we don't need to specify the type of q here because Rust
        // can infer the type based on how it's used in this function later on.
        // Full syntax:
        //      let mut q: Queue<u32> = Queue::new::<u32>();
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(3, q.len());
        assert_eq!(Some(3), q.pop());
        assert_eq!(2, q.len());
    }

    #[test]
    fn test_find_extrema() {
        let a = [0, -3, 0, 15, 48];
        let extrema = find_extrema(&a);
        assert_eq!(-3, *extrema.least);
        assert_eq!(48, *extrema.greatest);
    }

    #[test]
    fn test_common_traits() {
        // #[derive(Copy, Clone, Debug, PartialEq)]
        let p1 = Point { x: 3.5, y: 5.3 };
        let p2 = Point { x: 3.5, y: 5.3 };
        let p3 = p1.clone();
        let p4 = Point { x: 9.0, y: 6.3 };
        assert_eq!(p1, p2);
        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
        assert!(p1 != p4);
        println!("p2 is {:?}", p2);
    }
}
