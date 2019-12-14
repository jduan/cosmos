/// An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are
/// created using brackets [], and their size, which is known at compile time, is part of their
/// type signature [T; size].
///
/// Slices are similar to arrays, but their size is not known at compile time. Instead, a slice is
/// a two-word object, the first word is a pointer to the data, and the second word is the length
/// of the slice. Slices can be used to borrow a section of an array, and have the type signature
/// &[T].
pub fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_slices::analyze_slice;

    #[test]
    fn test_arrays_and_slices() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5]; // the type can be inferred
        assert_eq!(5, xs.len());

        // All elements can be initialized to the same value
        let ys: [i32; 500] = [0; 500];
        assert_eq!(500, ys.len());

        // Arrays are stack allocated
        assert_eq!(20, std::mem::size_of_val(&xs));

        // Arrays can be automatically borrowed as slices
        analyze_slice(&xs);

        // Slices can point to a section of an array
        analyze_slice(&ys[1..4]);

        // Out of bound indexing causes compile error
        // println!("{}", xs[5]);
    }
}
