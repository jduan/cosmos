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
