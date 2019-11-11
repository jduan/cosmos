pub fn run() {
    let v = build_vector();
    for e in v {
        println!("element: {}", e);
    }
    iterate();
    long_array();
    cannot_index_strings();
}

// Given the return type of this function, Rust can infer the type of "mut v".
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn iterate() {
    let n = -10;
    // it's ok even if n is a negative number
    for i in 0..n {
        println!("i is {}", i);
    }
}

fn long_array() {
    // create an array of 10 identical strings
    let mut a = ["Are we there yet?"; 10];
    a[0] = "Hello, world!";
    for i in a.iter() {
        println!("{:?}", i);
    }

    if a.len() > 5 {
        println!("array has more than 5 elements");
    } else {
        println!("array has <= 5 elements");
    }
}

fn cannot_index_strings() {
    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }

    println!("");
}
