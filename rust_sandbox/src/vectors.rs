pub fn run() {
    // Vectors are implemented using generics. Without type annotation, Rust
    // can't infer the type of v.
    let mut v: Vec<i32> = Vec::new();

    // vec! is a macro that creates a vector with initial values
    // Rust can infer v2's type.
    let v2 = vec![1, 2, 3];

    // append elements
    v.push(5);
    v.push(6);
    // remove elements
    v.pop();

    // access elemenets
    // accessing an element that's invalid will cause a panic!
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // type: Option<&T>
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    ownership();

    let v = vec![100, 32, 57];
    iterate_vec(&v);

    // modify elements while iterating
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * is the dereference operator
        *i += 50;
    }
    iterate_vec(&v);

    vec_of_enums();
}

fn ownership() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);

    // The line below would cause "v.push(6)" to fail to compile because
    // first is an immutable reference to the vector!
    // Why should a reference to the first element care about what changes at the end of the
    // vector? This error is due to the way vectors work: adding a new element onto the end of the
    // vector might require allocating new memory and copying the old elements to the new space, if
    // there isn’t enough room to put all the elements next to each other where the vector
    // currently is. In that case, the reference to the first element would be pointing to
    // deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    //
    // println!("The first element is {}", first);

    // this line works fine because "first" is already out of scope right after
    // it's defined. So it wouldn't prevent "v.push(6)"
    println!("The first element is {}", &v[0]);
}

fn iterate_vec(v: &Vec<i32>) {
    for i in v {
        println!("Vector element is {}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// Use an enum to store a vector of different types!
fn vec_of_enums() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("I'm an int {}", i),
            SpreadsheetCell::Float(f) => println!("I'm a float {}", f),
            SpreadsheetCell::Text(s) => println!("I'm a string {}", s),
        }
    }
}
