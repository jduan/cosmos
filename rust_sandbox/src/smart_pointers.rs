// This is so that you can refer Cons and Nil without prepending the List module
use std::ops::Deref;
use List::{Cons, Nil};

pub fn run() {
    box_simple_example();
    let lst = create_list();
    print_list(lst);
    dereference_operator();
    use_mybox();
    deref_coercion();
}

fn box_simple_example() {
    let b = Box::new(5);
    println!("b = {}", b);
    // When b goes out of scope at the end of the function, it will be deallocated.
    // The deallocation happens for the box (stored on the stack) and the data it
    // points to (stored on the heap);
}

enum List {
    Nil,
    Cons(i32, Box<List>),
}

fn create_list() -> List {
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
}

fn print_list(lst: List) {
    match lst {
        Nil => println!("End of list"),
        Cons(head, tail) => {
            println!("List element: {}", head);
            print_list(*tail);
        }
    }
}

fn dereference_operator() {
    let x = 5;
    // y is a reference to x
    let y = &x;

    assert_eq!(5, x);
    // * is the dereference operator
    assert_eq!(5, *y);

    // y is an instance of a Box pointing to the value of x
    let y = Box::new(x);
    // Box implements the Deref trait so you can use * to dereference y
    assert_eq!(5, *y);
}

// define our own Box type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Without the Deref trait, the compiler can only dereference & references. The deref method gives
// the compiler the ability to take a value of any type that implements Deref and call the deref
// method to get a & reference that it knows how to dereference.
impl<T> Deref for MyBox<T> {
    type Target = T;

    // returns the address of the embedded data so it can be dereferenced by *
    fn deref(&self) -> &T {
        &self.0
    }
}

fn use_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    // What happens behind the scene is Rust actually calls this code:
    // *(y.deref())
    assert_eq!(5, *y);

    // * doesn't take ownership of the dereferenced object so you can use * as many times as you
    // want.
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion() {
    hello("Rust");
    let m = MyBox::new(String::from("Rust"));

    // Here we’re calling the hello function with the argument &m, which is a reference to a
    // MyBox<String> value. Because we implemented the Deref trait on MyBox<T>,
    // Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string
    // slice. Rust calls deref again to turn the &String into &str, which matches the hello
    // function’s definition.
    hello(&m);

    // If Rust didn't implement deref coercion, we would have to write code like this:
    // The (*m) dereferences the MyBox<String> into a String. Then [..] converts the String
    // into a string slice. Then & creates a reference to the string slice.
    hello(&(*m)[..]);

    // Note that: When the Deref trait is defined for the types involved, Rust will analyze the
    // types and use Deref::deref as many times as necessary to get a reference to match the
    // parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at
    // compile time, so there is no runtime penalty for taking advantage of deref coercion!
}
