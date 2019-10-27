pub fn run() {
    // dangling_references();
    this_works();
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("The longest string is '{}'", result);
    calling_longest1();
}

// fn dangling_references() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     // x is out of scope here so the reference r isn't valid anymore!
//     // in other words: the subject of the reference doesn't live as long as the reference.
//     println!("r is {}", r);
// }

fn this_works() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    println!("x: {}", x);
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two
// parameters, both of which are string slices that live at least as long as lifetime 'a. The
// function signature also tells Rust that the string slice returned from the function will live at
// least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes of the references passed in.
// These constraints are what we want Rust to enforce. Remember, when we specify the lifetime
// parameters in this function signature, we’re not changing the lifetimes of any values passed in
// or returned. Rather, we’re specifying that the borrow checker should reject any values that
// don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly
// how long x and y will live, only that some scope can be substituted for 'a that will satisfy
// this signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function compiles.
fn calling_longest1() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("short string");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is '{}'", result);
    }
}

// This function doesn't compile! The error is "s2 doesn't live long enough".
// In order for "result" to be valid for the println! statement, "s2" would
// need to be valid until the end of the outer scope. Rust knows this because
// we annotated the lifetimes of the function parameters and return values using
// the same lifetime parameter 'a.
// fn calling_longest2() {
//     let s1 = String::from("long string is long");
//     let result;
//     {
//         let s2 = String::from("short string");
//         result = longest(s1.as_str(), s2.as_str());
//     }
//     println!("The longest string is '{}'", result);
// }
