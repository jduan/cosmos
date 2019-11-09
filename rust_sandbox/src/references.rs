use std::collections::HashMap;

pub fn run() {
    let mut table = create_table();
    show_table(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    assert_eq!(
        table["Caravaggio"],
        vec![
            "The Calling of St. Matthew".to_string(),
            "The Musicians".to_string(),
        ]
    );

    references_are_explicit();
    dot_operator();
    dot_operator2();
    reference_to_reference();
    compare_references();
    borrow_ref_of_any_expr();
    static WORTH_POINTING_AT: i32 = 1000;
    update_globa_var2(&WORTH_POINTING_AT);
}

// type alias
type Table = HashMap<String, Vec<String>>;

fn create_table() -> Table {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    table
}

// Immutable Reference
fn show_table(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("   {}", work);
        }
    }
}

// Mutable Reference
fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

// In Rust, references are created explicitly with the & operator, and dereferenced explicitly with
// the * operator.
fn references_are_explicit() {
    // immutable reference
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);

    // mutable reference
    let mut y = 32;
    // To borrow y as a mutable reference, y has to be defined as immutable as well
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
}

// Since references are so widely used in Rust, the . operator implicitly dereferences its left
// operand, if needed.
//
// See: https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082
//
// Other examples:
// 1. The println! macro expands to code that uses the dot operator, so it takes advantage of this
//    implicit dereference as well.
// 2. A "for" loop also expands to code that uses the dot operator. That's why you can iterate over
//    a reference to a collection without dereferencing it.
fn dot_operator() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let aria_ref = &aria;

    assert_eq!(aria_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*aria_ref).name, "Aria: The Animation");
}

// The . operator can also implicitly borrow a reference (immutable & mutable) to its left operand,
// if needed for a method call.
fn dot_operator2() {
    let mut v = vec![1973, 1968];
    v.sort();
    // same as above but much uglier
    (&mut v).sort();
    assert_eq!(v, vec![1968, 1973]);
}

struct Point {
    x: i32,
    y: i32,
}

fn reference_to_reference() {
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    // The type can be inferred by Rust.
    let rrr: &&&Point = &rr;

    // The dot operator follows as many references as it takes to find its target!
    assert_eq!(729, rr.y);
    assert_eq!(729, rrr.y);
}

fn compare_references() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    // This assertion succeeds, even though rrx and rry point at different values
    // (namely, rx and ry), because the == operator follows all the references and performs the
    // comparison on their final targets, x and y.
    assert_eq!(rrx, rry);

    // If you actually want to know whether two references point to the same memory, you can use
    // std::ptr::eq, which compares them as addresses:
    assert!(!std::ptr::eq(rx, ry));

    assert_eq!(rx, ry);
    assert_eq!(*rx, y);
    assert_eq!(*rrx, ry);
    // The line below doesn't compile because you can't compare references of different types.
    // assert_eq!(rrx, ry);
}

// Rust lets you borrow a reference to the value of any sort of expression at all.
// In situations like this, Rust simply creates an anonymous variable to hold the expression’s
// value, and makes the reference point to that. The lifetime of this anonymous variable depends on
// what you do with the reference:
//
// * If you immediately assign the reference to a variable in a let statement (or make it part of
// some struct or array that is being immediately assigned), then Rust makes the anonymous
// variable live as long as the variable the let initializes. In the example below, Rust
// would do this for the referent of r.
//
// * Otherwise, the anonymous variable lives to the end of the enclosing statement. In our
// example, the anonymous variable created to hold 1009 lasts only to the end of the
// assert_eq! statement.
fn borrow_ref_of_any_expr() {
    fn factorial(n: usize) -> usize {
        (1..(n + 1)).fold(1, |acc, c| acc * c)
    }

    // reference to a function call
    let r = &factorial(6);

    assert_eq!(r + &1009, 1729);
}

// Every variable needs to be initialized before you can use it.
// STASH is a "global" variable.
static mut STASH: &i32 = &128;

// This function doesn't compile!
// Since we must allow 'a to be any lifetime, things had better work out if it’s the smallest
// possible lifetime: one just enclosing the call to f.
// Since STASH lives for the program’s entire execution, the reference type it holds must have a
// lifetime of the same length; Rust calls this the 'static lifetime. But the lifetime of p’s
// reference is some 'a, which could be anything, as long as it encloses the call to f. So, Rust
// rejects our code.
// fn update_globa_var<'a>(p: &'a i32) {
//     // Mutable statics are inherently not thread-safe (after all, any thread can access a static at
//     // any time), and even in single-threaded programs, they can fall prey to other sorts of
//     // reentrancy problems. For these reasons, you may access a mutable static only within an
//     // unsafe block.
//     unsafe {
//         STASH = p;
//     }
// }

// This time, the function's signature spells out that p must be a reference with lifetime 'static,
// so there’s no longer any problem storing that in STASH. We can only apply f to references to
// other statics, but that’s the only thing that’s certain not to leave STASH dangling anyway
//
// Take a step back, though, and notice what happened to f’s signature as we amended our way to
// correctness: the original f(p: &i32) ended up as f(p: &'static i32). In other words, we were
// unable to write a function that stashed a reference in a global variable without reflecting that
// intention in the function’s signature. In Rust, a function’s signature always exposes the body’s
// behavior.
//
// Conversely, if we do see a function with a signature like g(p: &i32) (or with the lifetimes
// written out, g<'a>(p: &'a i32)), we can tell that it does not stash its argument p anywhere that
// will outlive the call. There’s no need to look into g’s definition; the signature alone tells us
// what g can and can’t do with its argument. This fact ends up being very useful when you’re
// trying to establish the safety of a call to the function.
fn update_globa_var2(p: &'static i32) {
    // Mutable statics are inherently not thread-safe (after all, any thread can access a static at
    // any time), and even in single-threaded programs, they can fall prey to other sorts of
    // reentrancy problems. For these reasons, you may access a mutable static only within an
    // unsafe block.
    unsafe {
        STASH = p;
    }
}
