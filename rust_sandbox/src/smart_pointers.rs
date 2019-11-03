// This is so that you can refer Cons and Nil without prepending the List module
use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};
use List2::{Cons as Cons2, Nil as Nil2};

pub fn run() {
    box_simple_example();
    let lst = create_list();
    print_list(lst);
    dereference_operator();
    use_mybox();
    deref_coercion();
    drop_trait();
    drop_early();
    reference_counting();
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

struct CustomSmartPointer {
    data: String,
}

// This trait lets you customize what happens when a value is about to go out of scope.
// Usually things like open files and network connections are handled there.
//
// In some languages, the programmer must call code to free memory or resources every time they
// finish using an instance of a smart pointer. If they forget, the system might become overloaded
// and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes
// out of scope, and the compiler will insert this code automatically. As a result, you don’t need
// to be careful about placing cleanup code everywhere in a program that an instance of a
// particular type is finished with—you still won’t leak resources!
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn drop_trait() {
    // Variables are dropped in the reverse order of their creation, so 'd' gets dropped before
    // 'c'.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
}

fn drop_early() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // Rust doesn't let us call `drop` explicitly because Rust would still automatically call
    // `drop` at the end of the function. That would be a "double free" error.
    // c.drop();

    // Instead, you can call the "drop" function from "std::mem::drop" part of prelude, to force
    // Rust to drop a value before the end of its scope. One example is when using smart pointers
    // to manage "locks". You might want to release the lock so other code in the same scope can
    // acquire the lock.
    drop(c);
    println!("CustomSmartPointer dropped before the end of the function.");
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn reference_counting() {
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    // The initial reference count is 1
    assert_eq!(1, Rc::strong_count(&a));

    let b = Cons2(3, Rc::clone(&a));
    // The reference count is 2 after one clone
    assert_eq!(2, Rc::strong_count(&a));

    {
        // The reference count is 3 after another clone
        let c = Cons2(4, Rc::clone(&a));
        assert_eq!(3, Rc::strong_count(&a));
    }

    // The reference count is back to 2 after c goes out of scope
    assert_eq!(2, Rc::strong_count(&a));

    // When the function finishes, a/b/c all go out of scope and the entire
    // list will be cleaned up completely.
}
