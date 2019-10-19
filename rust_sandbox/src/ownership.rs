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
