// This is so that you can refer Cons and Nil without prepending the List module
use List::{Cons, Nil};

pub fn run() {
    box_simple_example();
    let lst = create_list();
    print_list(lst);
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
