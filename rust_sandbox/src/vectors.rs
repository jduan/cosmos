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

pub fn ownership() {
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

pub fn iterate_vec(v: &Vec<i32>) {
    for i in v {
        println!("Vector element is {}", i);
    }
}

pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// Use an enum to store a vector of different types!
pub fn vec_of_enums() {
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

#[cfg(test)]
mod tests {
    #[test]
    fn create_vectors() {
        // this creates a vector of 100 int elements, all set to 0
        let v = vec![0; 100];

        assert_eq!(v[0], 0);
        assert_eq!(v.len(), 100);
    }

    #[test]
    fn create_vectors_from_iterators() {
        // You often need to supply the type when using "collect" because it can build many
        // differnet sorts of collections, not just vectors.
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, [0, 1, 2, 3, 4]);
    }

    // As with arrays, you can use "slice methods" on vectors.
    #[test]
    fn slice_methods() {
        let mut v = vec!["a man", "a plan", "a canal", "panama"];
        v.reverse();
        assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);
    }

    // A Vec<T> consists of 3 pieces of info:
    // * a pointer to the heap-allocated buffer that holds the elements
    // * the number of elements that the buffer has the capacity to store
    // * the number it actually contains now
    //
    // When the buffer has reached its capacity, adding another element to the vector entails
    // allocating a larger buffer, copying the present contents into it, updating the vector’s
    // pointer and capacity to describe the new buffer, and finally freeing the old one.
    //
    // If you know the number of elements a vector will need in advance, instead of Vec::new you
    // can call Vec::with_capacity to create a vector with a buffer large enough to hold them all,
    // right from the start; then, you can add the elements to the vector one at a time without
    // causing any reallocation.
    #[test]
    fn vector_capacity() {
        let mut v = Vec::with_capacity(2);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 2);

        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);

        v.push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 4);
    }

    // You can insert and remove elements wherever you like in a vector, although these operations
    // shift all the elements after the insertion point forward or backward, so they may be slow if
    // the vector is long.
    #[test]
    fn update_vector() {
        let mut v = vec![10, 20, 30, 40, 50];

        // Make the element at index 3 be 35.
        v.insert(3, 35);
        assert_eq!(v, [10, 20, 30, 35, 40, 50]);

        // Remove the element at index 1.
        v.remove(1);
        assert_eq!(v, [10, 30, 35, 40, 50]);

        // Pop items off the tail
        let mut v = vec!["carmen", "miranda"];
        assert_eq!(v.pop(), Some("miranda"));
        assert_eq!(v.pop(), Some("carmen"));
        assert_eq!(v.pop(), None);
    }
}
