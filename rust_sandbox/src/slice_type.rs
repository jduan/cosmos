pub fn run() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);
    println!("Index of the end of the first word is {}", word);

    // this empties the string, making it equal to ""
    s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let mut s2 = String::from("Hello, world!");
    let word2 = string_slice(&s2);

    // The following line won't compile because:
    // cannot borrow `s2` as mutable because it is also borrowed as immutable
    // s2.clear();
    println!("The first word is {}", word2);

    println!("The first word is {}", pass_string_slice(&s2[..]));
    let my_string_literal = "Hello, world";
    println!("The first word is {}", pass_string_slice(my_string_literal));

    // other kinds of slices
    let a = [1, 2, 3, 4, 5];
    let a_slice: &[i32] = &a[1..3];
}
// Return the index of the end of the first word.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Return a string slice "&str"
fn string_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// When passing strings to a function, favor "string slices"
// This function is identical to first_word except the function signature.
fn pass_string_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
