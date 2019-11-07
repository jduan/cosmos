pub fn run() {
    move_example();
    clone_example();

    let s = String::from("hello");
    takes_ownership(s);
    // s's value moves into the function and so is no longer valid here
    // println!("x is still available: {}", s);

    let x = 5;
    makes_copy(x);
    // x would move into the function, but i32 is a "Copy", so it's ok to still
    // use x afterwards.
    println!("x is still available: {}", x);

    let s2 = gives_ownership();
    println!("s2 is {}", s2);

    let s3 = String::from("Rust is great");
    println!("s3 is {}", takes_and_gives_back(s3));

    let s4 = String::from("This is a good day");
    let (s4, len) = calculate_length(s4);
    println!("Length of s4 is {}", len);

    let len2 = calculate_length2(&s4);
    println!("Length of s4 is {}", len2);

    let mut s5 = String::from("Hello");
    change(&mut s5);
    println!("s5 is now {}", s5);

    multiple_mut_refs();
    multiple_mut_refs2();

    cannot_move_indexed_content();
    work_with_indexed_content();
    loop_over_vector();
    replace_member();
    user_defined_types_are_not_copy_types();
    this_works_now();
}

fn move_example() {
    let s1 = String::from("hello");
    // s2 will be pointing to the object s1 used to point to.
    // s1 will go out of scope and become invalid!
    let s2 = s1;
    // The line below won't compile!
    // println!("original s1 is {}", s1);
}

fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("some_string is {}", some_string);
}
// Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("some_integer is {}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.

// This function moves its return value into the function that calls it.
fn gives_ownership() -> String {
    let some_string = String::from("John Rambo");
    some_string
}

// This function takes a String and returns it back
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// & is a reference, which allows you to refer to some value without
// taking ownership of it. Because it doesn't own the String object, the value
// it points to will not be dropped when the reference goes out of scope.
fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // the next line won't compile because you can't have multiple mutable references at the same
    // time
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn multiple_mut_refs2() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this scope so they are out of scope!

    let r3 = &mut s;
    println!("{}", r3);
}

struct Person {
    name: String,
    birth: i32,
}

fn misc() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
}

fn cannot_move_indexed_content() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // The line doesn't compile:
    // cannot move out of index of `std::vec::Vec<std::string::String>`
    // let third = v[2];

    // This works because "numbers" are copied instead of moved.
    let v2 = vec![1, 2, 3, 4, 5];
    let third = v2[2];
    println!("third element is {}", third);
}

fn work_with_indexed_content() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 1. pop a value off the end
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. swap one element with the one at the end
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. replace one element and return it
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

// Collection types like Vec also generally offer methods to consume all their elements in a loop
fn loop_over_vector() {
    let v = vec![
        "hello".to_string(),
        "world".to_string(),
        "everyone".to_string(),
    ];

    println!("Loop over a vector");

    // How does ownership work in a for loop?
    // When we pass v to the for loop, this "moves" the vector out of v, leaving v uninitialized.
    // The for loop's internal machinery takes ownership of the vector, and dissects it into its
    // elements. At each iteration, the loop moves another element to the variables "s". Since s
    // now owns the string, we're able to modify it in the loop before printing it. And since the
    // vector itself is no longer visible to the code, nothing can observe it mid-loop in some
    // partially emptied state.
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    // If you want to own v after the for loop, you need to borrow it.
    let v2 = vec![
        "hello".to_string(),
        "world".to_string(),
        "everyone".to_string(),
    ];
    for s in &v2 {
        println!("{}", s);
    }
    println!("v2 is still available after the loop: {:?}", v2);
}

struct Person2 {
    name: Option<String>,
    birth: i32,
}

// Replace a member of a struct
fn replace_member() {
    let mut composers = Vec::new();
    composers.push(Person2 {
        name: Some("Mozart".to_string()),
        birth: 1525,
    });
    // this doesn't work
    // let name = composers[0].name;

    let name = std::mem::replace(&mut composers[0].name, None);
    println!("name is {:?}", name);
}

struct Label {
    number: i32,
}

fn user_defined_types_are_not_copy_types() {
    fn print(label: Label) {
        println!("Label: {}", label.number);
    }

    let label = Label { number: 3 };
    print(label);
    // This line doesn't compile because the ownership of label has been moved to "print" in the
    // line above!
    // println!("My label number is {}", label.number);
}

// If all the fields of a user-defined struct are themselves Copy types, you can make the struct
// a Copy type too.
#[derive(Copy, Clone)]
struct Label2 {
    number: i32,
}

// The derive annotation doesn't work because String isn't a Copy type!
// #[derive(Copy, Clone)]
struct StringLabel {
    name: String,
}

fn this_works_now() {
    fn print(label: Label2) {
        println!("Label: {}", label.number);
    }

    let label = Label2 { number: 3 };
    print(label);

    // This works because the call of "print" above copies label.
    println!("My label number is {}", label.number);
}
