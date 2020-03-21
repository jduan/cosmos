/// A slice, written [T] without specifying the length, is a region of an array or vector. Since a
/// slice can be any length, slices can't be stored directly in variables or passed as function
/// arguments. Slices are always passed by references!
///
/// A slice is a "fat pointer": a two-word value comprising a pointer to the slice's first element,
/// and the number of elements in the slice.
///
pub fn run() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);
    println!("Index of the end of the first word is {}", word);

    // this empties the string, making it equal to ""
    s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s2 = String::from("Hello, world!");
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
    println!("a_slice: {:?}", a_slice);
}
// Return the index of the end of the first word.
pub fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Return a string slice "&str"
pub fn string_slice(s: &str) -> &str {
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
pub fn pass_string_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Because this function takes a slice reference as an argument, you can apply it to either a
// vector or an array.
pub fn iterate_slices(n: &[f64]) {
    for elt in n {
        println!("element is {}", elt);
    }
}

// A slice, written [T] without specifying the length, is a region of an array or vector. Since a
// slice can be any length, slices can’t be stored directly in variables or passed as function
// arguments. Slices are always passed by reference.

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn create_slices() {
        let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
        // similar to the vector above
        let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

        let sv: &[f64] = &v;
        let sa: &[f64] = &a;

        approx_eq!(f64, sv[0], 0.0);
        approx_eq!(f64, sv[1], 0.707);
        approx_eq!(f64, sa[0], 0.0);
        approx_eq!(f64, sa[1], 0.707);

        iterate_slices(&sv); // works on vectors
        iterate_slices(&sa); // works on arrays

        iterate_slices(&sv[0..2]); // print the first 2 elements of v
    }
}
