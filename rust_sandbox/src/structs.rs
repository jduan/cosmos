/// There are three types of structures ("structs") that can be created using the struct keyword:
///
///  1.  Tuple structs, which are, basically, named tuples.
///  2.  The classic C structs
///  3.  Unit structs, which are field-less, are useful for generics.

/// A tuple struct
pub struct Pair(i32, f32);

/// A unit struct
/// A unit struct is a type as well as an instance of that type.
pub struct Nil;

/// A classic C struct
// Deriving Common Traits for Struct Types
// Each of these traits can be implemented automatically for a struct, provided
// that each of its fields implements the trait.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

/// A C struct with pointers
#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    age: u8,
}

/// A struct can have another struct
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }
}

pub fn square(p: Point, width: f32) -> Rectangle {
    let p2 = Point {
        x: p.x + width,
        y: p.y + width,
    };
    Rectangle { p1: p, p2 }
}

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

    let rect = Rectangle2 {
        width: 30,
        height: 50,
    };
    println!("Area of rectangle {:#?} is {}", rect, area(&rect));
}

pub struct User {
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
pub struct Color(i32, i32, i32);
// fields are private by default
pub struct Bounds(pub usize, pub usize);

pub fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

pub fn update_struct(user: User) -> User {
    User {
        email: String::from("joel@airbnb.com"),
        username: String::from("Joel"),
        // the rest fields come from "user"
        ..user
    }
}

#[derive(Debug)]
pub struct Rectangle2 {
    width: u32,
    height: u32,
}

pub fn area(rect: &Rectangle2) -> u32 {
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

pub struct Extrema<'a> {
    greatest: &'a i32,
    least: &'a i32,
}

// Accordding "elision rule", you can omit the lifetime parameter for this
// function.
// fn find_extrema<'a>(slice: &'a [i32]) -> Extrema<'a> {
pub fn find_extrema(slice: &[i32]) -> Extrema {
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

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn test_person() {
        let person = Person {
            name: "Peter",
            age: 27,
        };
        assert_eq!(27, person.age);
    }

    #[test]
    fn test_point() {
        let point = Point { x: 0.3, y: 0.4 };
        assert!(approx_eq!(f32, 0.7, point.x + point.y));

        // Create another point from an existing point
        let point2 = Point { x: 0.1, ..point };
        assert!(approx_eq!(f32, 0.5, point2.x + point2.y));

        // destruct
        let Point { x: my_x, y: my_y } = point2;
        assert!(approx_eq!(f32, 0.1, my_x));
        assert!(approx_eq!(f32, 0.4, my_y));

        let rect = Rectangle {
            p1: point,
            p2: Point { x: 0.9, y: 0.8 },
        };
        assert_eq!(0.24, rect.area());
    }

    #[test]
    fn test_tuple_struct() {
        let pair = Pair(1, 0.1);
        assert_eq!(1, pair.0);

        let Pair(integer, _decimal) = pair;
        assert_eq!(1, integer);
    }

    #[test]
    fn test_square() {
        let point = Point { x: 1.0, y: 1.0 };
        let rect = square(point, 2.0);
        assert!(approx_eq!(f32, 4.0, rect.area()));
    }

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
